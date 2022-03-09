/*! Models relating to an in-progress zombiesplit session.

This module contains the bulk of the model surface of the zombiesplit server, covering:

- attempts, which are in-progress runs;
- sessions, which manage said runs and expose various API surfaces for handling them;
- actions, which form the command surface of sessions;
- observers, which form an observer pattern based API for monitoring changes to a session;
- sinks, which receive runs after the user resets the session.
*/
pub mod action;
pub mod attempt;
pub mod event;
pub mod sink;
pub mod split;
pub mod state;

use event::{split::Observer as SO, time::Observer as TO};

use super::{
    timing::{aggregate, comparison, comparison::provider, Comparison},
    Time,
};

pub use action::Action;
pub use attempt::Attempt;
pub use event::observer::Observer;
pub use event::Event;
pub use sink::Sink;
pub use split::Split;
pub use state::State;

/// A session over run attempts.
///
/// A session holds state in the form of a [State].
///
/// It also has zero or more observers attached that can be sent information
/// about the run's progress, and a comparison provider.  The latter feeds into
/// the session's lifetime.
pub struct Session<'cmp, 'obs, O> {
    /// The state of the session.
    state: State,
    /// The observer attached to the session (which, itself, may be observable).
    observer: &'obs O,
    /// The function for timestamping outgoing runs.
    timestamper: fn() -> chrono::DateTime<chrono::Utc>,

    //
    // Integrations with the historical model
    //

    // TODO(@MattWindsor91): refactor those into a separate struct?
    /// The sink attached to the session, for emitting saved runs.
    sink: Box<dyn sink::Sink>,
    /// The comparison provider.
    comparator: Box<dyn comparison::Provider + 'cmp>,
}

impl<'cmp, 'obs, O: Observer> action::Handler for Session<'cmp, 'obs, O> {
    // This error may change later.
    type Error = std::convert::Infallible;

    fn dump(&mut self) -> Result<State, Self::Error> {
        Ok(self.state.clone())
    }

    fn handle(&mut self, action: action::Action) -> Result<(), Self::Error> {
        match action {
            action::Action::NewRun(dest) => self.reset(dest),
            action::Action::Pop(s, action::Pop::One) => self.pop_from(s),
            action::Action::Pop(s, action::Pop::All) => self.clear_at(s),
            action::Action::Push(s, t) => self.push_to(s, t),
        };
        Ok(())
    }
}

impl<'cmp, 'obs, 'snk, O: Observer> Session<'cmp, 'obs, O> {
    /// Starts a new session with a given set of metadata and starting run.
    #[must_use]
    pub fn new(run: Attempt, observer: &'obs O) -> Self {
        Self {
            state: State::new(run, Comparison::default()),
            observer,
            sink: Box::new(sink::Null),
            timestamper: chrono::Utc::now,
            comparator: Box::new(provider::Null),
        }
    }

    // TODO(@MattWindsor91): replace these 'set_' functions with a builder.

    /// Replaces the session's timestamper with a different function.
    ///
    /// Useful for stubbing out time when testing.
    pub fn set_timestamper(&mut self, ts: fn() -> chrono::DateTime<chrono::Utc>) {
        self.timestamper = ts;
    }

    /// Replaces the session's comparison provider with a different one.
    ///
    /// By default, the session doesn't have comparisons set up, so this will
    /// need to be done to get comparisons working.
    ///
    /// Triggers an immediate comparison reset.
    pub fn set_comparison_provider(&mut self, p: Box<dyn provider::Provider + 'cmp>) {
        self.comparator = p;
        self.refresh_comparison();
    }

    /// Replaces the session's run sink with a different one.
    ///
    /// By default, the session doesn't have comparisons set up, so this will
    /// need to be done to get comparisons working.
    pub fn set_sink(&mut self, s: Box<dyn sink::Sink>) {
        self.sink = s;
    }

    /// Asks the comparison provider for an updated comparison.
    ///
    /// This should occur when the run is reset, in case the outgoing run has
    /// changed the comparisons.
    fn refresh_comparison(&mut self) {
        // TODO(@MattWindsor91): abort on error?
        match self.comparator.comparison() {
            Ok(Some(c)) => {
                self.state.comparison = c;
                self.observe_comparison();
            }
            Ok(None) => {}
            Err(e) => {
                log::error!("couldn't get comparison: {e}");
            }
        }
    }

    fn observe_reset(&self) {
        self.observer.observe(Event::Reset(self.state.run.info));
    }

    /// Observes notes for each split, notifying all observers.
    ///
    /// The notes will have been recalculated by the state before this is called.
    fn observe_notes(&self) {
        // TODO(@MattWindsor91): start from a particular split, to avoid redundancy?
        for (short, note) in &self.state.notes {
            self.observer
                .observe_split(*short, event::split::Split::Pace(note.pace));
            self.observer.observe_aggregate_set(
                *short,
                note.aggregates,
                aggregate::Source::Attempt,
            );
        }

        let pace = self.state.total.map(|x| x.pace).unwrap_or_default();
        let time = self.state.total.map(|x| x.time);
        self.observer
            .observe(Event::Total(event::Total::Attempt(pace), time));
    }

    /// Observes the contents of a comparison.
    ///
    /// This lets the user interface know, for each splits, which times we are
    /// running against.
    fn observe_comparison(&self) {
        self.observe_comparison_run();
        self.observe_comparison_splits();
    }

    /// Observes comparison data for the run as a whole.
    fn observe_comparison_run(&self) {
        for (ty, val) in self.state.comparison.run.totals() {
            self.observer
                .observe(Event::Total(event::Total::Comparison(ty), val));
        }
    }

    /// Observes comparison data for each split in the run.
    fn observe_comparison_splits(&self) {
        for split in self.state.run.splits.iter() {
            let short = split.info.short;
            if let Some(s) = self.state.comparison.aggregate_for(short) {
                self.observer
                    .observe_aggregate_set(short, *s, aggregate::Source::Comparison);
            }
        }
    }

    fn reset(&mut self, dest: action::OldDestination) {
        self.handle_old_run(dest);
        self.state.run.reset();
        // Important that this happens AFTER the run is reset, so the new attempt info is sent.
        self.observe_reset();
        self.refresh_comparison();
    }

    fn handle_old_run(&mut self, dest: action::OldDestination) {
        match dest {
            action::OldDestination::Save => self.send_run_to_sink(),
            action::OldDestination::Discard => (),
        }
    }

    fn send_run_to_sink(&mut self) {
        if let Some(r) = self.state.run.as_historic((self.timestamper)()) {
            if let Err(e) = self.sink.accept(r) {
                log::warn!("couldn't save run: {e}");
            }
        }
    }

    fn clear_at(&mut self, split: impl split::Locator) {
        if let Some(_s) = self.state.clear_at(split) {
            // TODO(@MattWindsor91): observe
            self.observe_notes();
        }
    }

    fn push_to(&mut self, split: impl split::Locator, time: Time) {
        if let Some(short) = self.state.push_to(split, time) {
            self.observer
                .observe_time(short, time, event::time::Time::Pushed);
            self.observe_notes();
        }
    }

    fn pop_from(&mut self, split: impl split::Locator) {
        if let Some((short, time)) = self.state.pop_from(split) {
            self.observer
                .observe_time(short, time, event::time::Time::Popped);
            self.observe_notes();
        }
    }
}
