//! The [Session] type and related code.

use crate::model::{game::category::ShortDescriptor, history};

use super::super::{
    comparison::{self, pace, Comparison},
    game::category,
    time::Time,
};
use super::{observer::Observer, run::Status, split::Set, Run};

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
    observers: Vec<Box<dyn Observer + 'a>>,
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
            observers: vec![],
            timestamper: chrono::Utc::now,
            comparator: Box::new(comparison::NullProvider),
        }
    }

    /// Replaces the session's timestamper with a different function.
    ///
    /// Useful for stubbing out time when testing.
    pub fn set_timestamper(&mut self, ts: fn() -> chrono::DateTime<chrono::Utc>) {
        self.timestamper = ts
    }

    /// Replaces the session's comparison provider with a different one.
    ///
    /// By default, the session doesn't have comparisons set up, so this will
    /// need to be done to get comparisons working.
    ///
    /// Triggers an immediate comparison reset.
    pub fn set_comparison_provider(&mut self, p: Box<dyn comparison::Provider + 'a>) {
        self.comparator = p;
        self.refresh_comparison()
    }

    /// Adds an observer to this session.
    pub fn add_observer(&mut self, o: Box<dyn Observer + 'a>) {
        self.observers.push(o)
    }

    /// Gets the paced time for the split at `split`.
    /// Said pace is made up of the split and run-so-far paces.
    #[must_use]
    pub fn paced_time_at(&self, split: usize) -> pace::Pair {
        pace::Pair {
            split: self.split_paced_time_at(split),
            run_so_far: self.run_paced_time_at(split),
        }
    }

    /// Gets the current run's attempt number.
    #[must_use]
    pub fn attempt(&self) -> usize {
        self.run.attempt
    }

    fn run_paced_time_at(&self, split: usize) -> pace::PacedTime {
        self.comparison.run_paced_time(split, self.total_at(split))
    }

    fn split_paced_time_at(&self, split: usize) -> pace::PacedTime {
        self.comparison.split_paced_time(split, self.time_at(split))
    }

    /// Gets the comparison time at `split`.
    #[must_use]
    pub fn comparison_time_at(&self, split: usize) -> Option<Time> {
        self.comparison.split_comparison_time(split)
    }

    /// Converts this session's current run, if any, to a historic run.
    ///
    /// Returns `None` if there is no started run.
    #[must_use]
    pub fn run_as_historic(&self) -> Option<history::run::FullyTimed<ShortDescriptor>> {
        match self.run.status() {
            Status::NotStarted => None,
            Status::Complete => Some(self.run_as_historic_with_completion(true)),
            Status::Incomplete => Some(self.run_as_historic_with_completion(false)),
        }
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

    fn observe_reset(&self) {
        for o in &self.observers {
            o.on_reset(&self)
        }
    }

    /// Asks the comparison provider for an updated comparison.
    ///
    /// This should occur when the run is reset, in case the outgoing run has
    /// changed the comparisons.
    fn refresh_comparison(&mut self) {
        if let Some(c) = self.comparator.comparison() {
            self.comparison = c
        }
    }
}

/// The session exposes its underlying run as a split set.
impl<'a> Set for Session<'a> {
    fn reset(&mut self) {
        self.observe_reset();
        self.run.reset();
        self.refresh_comparison()
    }

    fn clear_at(&mut self, split: usize) {
        self.run.clear_at(split)
    }

    fn push_to(&mut self, split: usize, time: Time) {
        self.run.push_to(split, time)
    }

    fn pop_from(&mut self, split: usize) -> Option<Time> {
        self.run.pop_from(split)
    }

    fn total_at(&self, split: usize) -> Time {
        self.run.total_at(split)
    }

    fn num_times_at(&self, split: usize) -> usize {
        self.run.num_times_at(split)
    }

    fn time_at(&self, split: usize) -> Time {
        self.run.time_at(split)
    }

    fn num_splits(&self) -> usize {
        self.run.num_splits()
    }

    fn name_at(&self, split: usize) -> &str {
        self.run.name_at(split)
    }
}
