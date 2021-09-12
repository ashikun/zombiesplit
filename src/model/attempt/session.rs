//! The [Session] type and related code.

use super::{
    super::{
        aggregate,
        comparison::{self, Comparison},
        game::category,
        history, short, Time,
    },
    observer::{self, split::Observer as SO, time::Observer as TO},
    split, Observer, Run,
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

    /// Converts this session's current run, if any, to a historic run.
    ///
    /// Returns `None` if there is no started run.
    #[must_use]
    pub fn run_as_historic(&self) -> Option<history::run::FullyTimed<category::ShortDescriptor>> {
        self.run
            .status()
            .to_completeness()
            .map(|c| self.run_as_historic_with_completion(c))
    }

    fn run_as_historic_with_completion(
        &self,
        was_completed: bool,
    ) -> history::run::FullyTimed<category::ShortDescriptor> {
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
        for split in self.run.splits.iter() {
            self.observers.observe(observer::Event::AddSplit(
                split.info.short,
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
    fn recalculate_and_observe_splits(&self, split: short::Name) {
        // TODO(@MattWindsor91): start from split
        for (short, agg) in self.run.splits.aggregates() {
            let pace = self.comparison.pace(short, agg);

            self.observers
                .observe_split(split, observer::split::Event::Pace(pace));
            self.observe_aggregate(short, agg, aggregate::Source::Attempt);
        }
    }

    /// Observes the contents of a comparison.
    ///
    /// This lets the user interface know, for each splits, which times we are
    /// running against.
    fn observe_comparison(&self) {
        for split in self.run.splits.iter() {
            let short = split.info.short;
            if let Some(s) = self.comparison.aggregate_for(short) {
                self.observe_aggregate(short, *s, aggregate::Source::Comparison);
            }
        }
    }

    /// Observes an aggregate pair `pair` for aggregate source `source` for split `split`.
    fn observe_aggregate(
        &self,
        split: short::Name,
        pair: aggregate::Set,
        source: aggregate::Source,
    ) {
        self.observe_aggregate_part(split, pair.split, source.with(aggregate::Scope::Split));
        self.observe_aggregate_part(
            split,
            pair.cumulative,
            source.with(aggregate::Scope::Cumulative),
        );
    }

    fn observe_aggregate_part(
        &self,
        split: short::Name,
        time: Option<Time>,
        kind: aggregate::Kind,
    ) {
        if let Some(time) = time {
            self.observers
                .observe_time(split, time, observer::time::Event::Aggregate(kind));
        }
    }

    pub fn reset(&mut self) {
        self.observe_reset();
        self.run.reset();
        self.observe_attempt();
        self.refresh_comparison();
    }

    pub fn clear_at(&mut self, split: impl split::Locator) {
        if let Some(s) = self.locate_split_mut(split) {
            s.clear();
            // TODO(@MattWindsor91): observe
        }
    }

    pub fn push_to(&mut self, split: impl split::Locator, time: Time) {
        if let Some(s) = self.locate_split_mut(split) {
            s.push(time);
            let short = s.info.short;
            self.observers
                .observe_time(short, time, observer::time::Event::Pushed);
            self.recalculate_and_observe_splits(short);
        }
    }

    pub fn pop_from(&mut self, split: impl split::Locator) -> Option<Time> {
        let splits = &mut self.run.splits;
        split
            .locate_mut(splits)
            .and_then(|s| {
                let short = s.info.short;
                s.pop().map(|time| (short, time))
            })
            .map(|(short, time)| {
                self.observers
                    .observe_time(short, time, observer::time::Event::Popped);
                self.recalculate_and_observe_splits(short);
                time
            })
    }

    fn locate_split_mut(&mut self, split: impl split::Locator) -> Option<&mut split::Split> {
        split.locate_mut(&mut self.run.splits)
    }
}
