//! The [Session] type and related code.

use super::{
    super::{
        game::category::Info,
        timing::{
            aggregate,
            comparison::{pace, provider, Comparison},
            Time,
        },
    },
    action,
    observer::{self, split::Observer as SO, time::Observer as TO, Event},
    sink, split, Observer, Run,
};

/// Holds all data for an attempt session.
///
/// A session consists of category information, the current run being
/// attempted, and any comparison data being worked against.
///
/// It also has zero or more observers attached that can be sent information
/// about the run's progress, and a comparison provider.  The latter feeds into
/// the session's lifetime.
pub struct Session<'cmp, 'obs, O> {
    /// The current run.
    run: Run,
    /// Comparison data for the game/category currently being run.
    comparison: Comparison,
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
    comparator: Box<dyn provider::Provider + 'cmp>,
}

impl<'cmp, 'obs, O: Observer> action::Handler for Session<'cmp, 'obs, O> {
    fn handle(&mut self, action: action::Action) {
        match action {
            action::Action::Dump => self.dump_to_observers(),
            action::Action::Clear(s) => self.clear_at(s),
            action::Action::NewRun => self.reset(),
            action::Action::Pop(s) => self.pop_from(s),
            action::Action::Push(s, t) => self.push_to(s, t),
        }
    }
}

impl<'cmp, 'obs, 'snk, O: Observer> Session<'cmp, 'obs, O> {
    /// Starts a new session with a given set of metadata and starting run.
    #[must_use]
    pub fn new(run: Run, observer: &'obs O) -> Self {
        Self {
            run,
            comparison: Comparison::default(),
            observer,
            sink: Box::new(sink::Null),
            timestamper: chrono::Utc::now,
            comparator: Box::new(provider::Null),
        }
    }

    /// Gets a reference to the current run's metadata.
    #[must_use]
    pub fn metadata(&self) -> &Info {
        &self.run.metadata
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

    /// Dumps a clone of the current run.
    #[must_use]
    pub fn dump(&self) -> Run {
        self.run.clone()
    }

    /// Asks the comparison provider for an updated comparison.
    ///
    /// This should occur when the run is reset, in case the outgoing run has
    /// changed the comparisons.
    fn refresh_comparison(&mut self) {
        // TODO(@MattWindsor91): abort on error?
        match self.comparator.comparison() {
            Ok(Some(c)) => {
                self.comparison = c;
                self.observe_comparison();
            }
            Ok(None) => {}
            Err(e) => {
                log::error!("couldn't get comparison: {e}");
            }
        }
    }

    /// Dumps initial session information to the observers.
    ///
    /// This should only be called once, as it might not be idempotent.
    fn dump_to_observers(&self) {
        // TODO(@MattWindsor91): make this idempotent.
        self.observe_game_category();
        self.observe_splits();
        self.observe_attempt();
        self.observe_comparison();
    }

    /// Sends information about each split to the observers.
    fn observe_splits(&self) {
        self.observer
            .observe(Event::NumSplits(self.run.splits.len()));
        for (index, split) in self.run.splits.iter().enumerate() {
            self.observer.observe_split(
                split.info.short,
                observer::split::Event::Init {
                    index,
                    name: split.info.name.clone(),
                },
            );
        }
    }

    fn observe_reset(&self) {
        self.observer.observe(Event::Reset);
    }

    fn observe_attempt(&self) {
        self.observer.observe(Event::Attempt(self.run.attempt));
    }

    fn observe_game_category(&self) {
        self.observer
            .observe(Event::GameCategory(self.run.metadata.clone()));
    }

    /// Observes all paces and aggregates for each split, notifying all
    /// observers.
    fn observe_paces_and_aggregates(&self) {
        // TODO(@MattWindsor91): start from a particular split, to avoid
        // redundancy?

        let mut total = pace::PacedTime::default();

        for (split, agg) in self.run.splits.aggregates() {
            let pace = self.split_pace(split, agg);
            let short = split.info.short;
            self.observer
                .observe_split(short, observer::split::Event::Pace(pace));
            self.observer
                .observe_aggregate_set(short, agg, aggregate::Source::Attempt);

            if total.time < agg.cumulative {
                total = pace::PacedTime {
                    pace: pace.overall(),
                    time: agg.cumulative,
                }
            }
        }

        self.observer
            .observe(Event::Total(total, aggregate::Source::Attempt));
    }

    fn split_pace(&self, split: &super::Split, agg: aggregate::Set) -> pace::SplitInRun {
        if split.num_times() == 0 {
            pace::SplitInRun::Inconclusive
        } else {
            self.comparison.pace(split.info.short, agg)
        }
    }

    /// Observes the contents of a comparison.
    ///
    /// This lets the user interface know, for each splits, which times we are
    /// running against.
    fn observe_comparison(&self) {
        let Comparison {
            total, sum_of_best, ..
        } = self.comparison;

        // TODO(@MattWindsor91): wrapping this in a PacedTime is a bit silly.
        self.observer.observe(Event::Total(
            pace::PacedTime::inconclusive(total),
            aggregate::Source::Comparison,
        ));
        self.observer.observe(Event::SumOfBest(sum_of_best));
        self.observe_comparison_splits();
    }

    /// Observes comparison data for each split in the run.
    fn observe_comparison_splits(&self) {
        for split in self.run.splits.iter() {
            let short = split.info.short;
            if let Some(s) = self.comparison.aggregate_for(short) {
                self.observer
                    .observe_aggregate_set(short, *s, aggregate::Source::Comparison);
            }
        }
    }

    fn reset(&mut self) {
        self.observe_reset();
        self.send_run_to_sink();
        self.run.reset();
        self.observe_attempt();
        self.refresh_comparison();
    }

    fn send_run_to_sink(&mut self) {
        if let Some(r) = self.run.as_historic((self.timestamper)()) {
            if let Err(e) = self.sink.accept(r) {
                log::warn!("couldn't save run: {e}");
            }
        }
    }

    fn clear_at(&mut self, split: impl split::Locator) {
        if let Some(s) = self.run.splits.get_mut(split) {
            s.clear();
            // TODO(@MattWindsor91): observe
        }
    }

    fn push_to(&mut self, split: impl split::Locator, time: Time) {
        if let Some(s) = self.run.splits.get_mut(split) {
            s.push(time);
            let short = s.info.short;
            self.observer
                .observe_time(short, time, observer::time::Event::Pushed);
            self.observe_paces_and_aggregates();
        }
    }

    fn pop_from(&mut self, split: impl split::Locator) {
        if let Some((short, time)) = self.run.splits.get_mut(split).and_then(|s| {
            let short = s.info.short;
            s.pop().map(|time| (short, time))
        }) {
            self.observer
                .observe_time(short, time, observer::time::Event::Popped);
            self.observe_paces_and_aggregates();
        }
    }
}
