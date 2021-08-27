//! The [Session] type and related code.

use crate::model::{game::category::ShortDescriptor, history};

use super::{
    super::{
        comparison::{self, pace, Comparison},
        game::category,
        time::Time,
    },
    observer::{self, aggregate, split},
    split::Set,
    Observer, Run,
};

/// Holds all data for an attempt session.
///
/// A session consists of category information, the current run being
/// attempted, and any comparison data being worked against.
///
/// It also has zero or more observers attached that can be sent information
/// about the run's progress, and a comparison provider.  Both feed into the
/// session's lifetime.
pub struct Session<'a> {
    /// Metadata for the game/category currently being run.
    pub metadata: category::Info,
    /// The current run.
    run: Run,
    /// Comparison data for the game/category currently being run.
    comparison: Comparison,
    /// Any observers attached to the session.
    pub observers: observer::Mux<'a>,
    /// The function for timestamping outgoing runs.
    timestamper: fn() -> chrono::DateTime<chrono::Utc>,
    /// The comparison provider.
    comparator: Box<dyn comparison::Provider + 'a>,
}

type SplitId = usize;

impl<'a> Session<'a> {
    /// Starts a new session with a given set of metadata and starting run.
    #[must_use]
    pub fn new(metadata: category::Info, run: Run) -> Self {
        Self {
            metadata,
            run,
            comparison: Comparison::default(),
            observers: observer::Mux::default(),
            timestamper: chrono::Utc::now,
            comparator: Box::new(comparison::NullProvider),
        }
    }

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
    pub fn set_comparison_provider(&mut self, p: Box<dyn comparison::Provider + 'a>) {
        self.comparator = p;
        self.refresh_comparison();
    }

    /// Gets the paced time for the split at `split`.
    /// Said pace is made up of the split and run-so-far paces.
    #[must_use]
    pub fn paced_time_at(&self, split: SplitId) -> pace::Pair {
        pace::Pair {
            split: self.split_paced_time_at(split),
            run_so_far: self.run_paced_time_at(split),
        }
    }

    fn run_paced_time_at(&self, split: SplitId) -> pace::PacedTime {
        self.comparison
            .run_paced_time(split, self.run.cumulative_at(split))
    }

    fn split_paced_time_at(&self, split: SplitId) -> pace::PacedTime {
        self.comparison
            .split_paced_time(split, self.run.time_at(split))
    }

    /// Gets the comparison time at `split`.
    #[must_use]
    pub fn comparison_time_at(&self, split: SplitId) -> Option<Time> {
        self.comparison.split_comparison_time(split)
    }

    /// Converts this session's current run, if any, to a historic run.
    ///
    /// Returns `None` if there is no started run.
    #[must_use]
    pub fn run_as_historic(&self) -> Option<history::run::FullyTimed<ShortDescriptor>> {
        self.run
            .status()
            .to_completeness()
            .map(|c| self.run_as_historic_with_completion(c))
    }

    fn run_as_historic_with_completion(
        &self,
        was_completed: bool,
    ) -> history::run::FullyTimed<ShortDescriptor> {
        history::run::FullyTimed {
            category_locator: self.metadata.short.clone(),
            was_completed,
            date: (self.timestamper)(),
            timing: self.run.timing_as_historic(),
        }
    }

    /// Asks the comparison provider for an updated comparison.
    ///
    /// This should occur when the run is reset, in case the outgoing run has
    /// changed the comparisons.
    fn refresh_comparison(&mut self) {
        if let Some(c) = self.comparator.comparison() {
            self.comparison = c;
        }
        self.observe_comparison();
    }

    /// Dumps initial session information to the observers.
    ///
    /// This should only be called once, as it might not be idempotent.
    pub fn dump_to_observers(&self) {
        self.observe_game_category();
        self.observe_splits();
        self.observe_attempt();
        self.observe_comparison();
    }

    /// Sends information about each split to the observers.
    fn observe_splits(&self) {
        // TODO(@MattWindsor91): see if I can lifetime this better.
        for split in &self.run.splits {
            self.observers.observe(observer::Event::AddSplit(
                split.info.short.clone(),
                split.info.name.clone(),
            ));
        }
    }

    fn observe_reset(&self) {
        self.observers
            .observe(observer::Event::Reset(self.run_as_historic()));
    }

    fn observe_attempt(&self) {
        self.observers
            .observe(observer::Event::Attempt(self.run.attempt));
    }

    fn observe_game_category(&self) {
        self.observers
            .observe(observer::Event::GameCategory(self.metadata.clone()));
    }

    /// Recalculates times and pacings for every split below and
    /// including `split`, notifying all observers.
    ///
    /// We assume the caller has already updated the split, but not observed it
    /// yet.
    fn recalculate_and_observe_splits(&self, split: SplitId) {
        // TODO(@MattWindsor91): update run cumulatives and paces here, rather than
        // calculating them afresh every time.
        for i in split..=self.run.num_splits() {
            let pt = self.paced_time_at(split);
            self.observe_split(split, split::Event::Pace(pt.split_in_run_pace()));
            self.observe_attempt_split_time(i, pt.split.time);
            self.observe_attempt_cumulative(i, pt.run_so_far.time);
        }
    }

    fn observe_attempt_split_time(&self, split: SplitId, time: Time) {
        self.observe_aggregate(
            split,
            time,
            aggregate::Kind::attempt(aggregate::Scope::Split),
        );
    }

    fn observe_attempt_cumulative(&self, split: SplitId, time: Time) {
        self.observe_aggregate(
            split,
            time,
            aggregate::Kind::attempt(aggregate::Scope::Cumulative),
        );
    }

    fn observe_comparison(&self) {
        for i in 0..=self.run.num_splits() {
            self.observe_comparison_split_time(i);
            self.observe_comparison_cumulative(i);
        }
    }

    fn observe_comparison_split_time(&self, split: SplitId) {
        if let Some(s) = self.comparison.split(split).and_then(|x| x.in_run) {
            self.observe_aggregate(
                split,
                s.time,
                aggregate::Kind::comparison(aggregate::Scope::Split),
            );
        }
    }

    fn observe_comparison_cumulative(&self, split: SplitId) {
        if let Some(s) = self.comparison.split(split).and_then(|x| x.in_run) {
            self.observe_aggregate(
                split,
                s.cumulative,
                aggregate::Kind::comparison(aggregate::Scope::Cumulative),
            );
        }
    }

    fn observe_aggregate(&self, split: SplitId, time: Time, kind: aggregate::Kind) {
        self.observe_split(
            split,
            split::Event::Time(time, split::Time::Aggregate(kind)),
        );
    }

    fn observe_split(&self, split: SplitId, event: split::Event) {
        self.observers.observe(observer::Event::Split(
            self.split_from_position(split),
            event,
        ));
    }

    fn split_from_position(&self, pos: usize) -> String {
        // TODO(@MattWindsor91): this should ideally be temporary.
        self.run
            .splits
            .get(pos)
            .map_or_else(String::default, |x| x.info.short.clone())
    }
}

/// The session exposes its underlying run as a split set.
impl<'a> Set for Session<'a> {
    fn reset(&mut self) {
        self.observe_reset();
        self.run.reset();
        self.observe_attempt();
        self.refresh_comparison();
    }

    fn clear_at(&mut self, split: SplitId) {
        self.run.clear_at(split);
    }

    fn push_to(&mut self, split: SplitId, time: Time) {
        self.run.push_to(split, time);
        self.observe_split(split, split::Event::Time(time, split::Time::Pushed));
        self.recalculate_and_observe_splits(split);
    }

    fn pop_from(&mut self, split: SplitId) -> Option<Time> {
        self.run.pop_from(split).map(|time| {
            self.observe_split(split, split::Event::Time(time, split::Time::Popped));
            self.recalculate_and_observe_splits(split);
            time
        })
    }
}
