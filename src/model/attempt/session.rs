//! The [Session] type and related code.

use crate::model::{game::category::ShortDescriptor, history::TimedRun};

use super::super::{
    comparison::{pace, Comparison},
    game::category,
    time::Time,
};
use super::{observer::Observer, run::Status, split::Set, Run};

/// Holds all data for an attempt session.
///
/// A session consists of category information, the current run being
/// attempted, and any comparison data being worked against.
///
/// It also has zero or more observers attached that can
pub struct Session {
    /// Metadata for the game/category currently being run.
    pub metadata: category::Info,
    /// The current run.
    run: Run,
    /// Comparison data for the game/category currently being run.
    comparisons: Comparison,
    observers: Vec<Box<dyn Observer>>,
}

impl Session {
    /// Starts a new session with a given set of metadata and starting run.
    #[must_use]
    pub fn new(metadata: category::Info, run: Run) -> Self {
        Self {
            metadata,
            run,
            comparisons: Comparison { splits: vec![] },
            observers: vec![],
        }
    }

    /// Adds an observer to this session.
    pub fn add_observer(&mut self, o: Box<dyn Observer>) {
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
        self.comparisons
            .run_paced_time_at(split, self.total_at(split))
    }

    fn split_paced_time_at(&self, split: usize) -> pace::PacedTime {
        self.comparisons
            .split_paced_time_at(split, self.time_at(split))
    }

    /// Converts this session's current run, if any, to a historic run.
    ///
    /// Returns `None` if there is no started run.
    #[must_use]
    pub fn run_as_historic(&self) -> Option<TimedRun<ShortDescriptor>> {
        match self.run.status() {
            Status::NotStarted => None,
            Status::Complete => Some(self.run_as_historic_with_completion(true)),
            Status::Incomplete => Some(self.run_as_historic_with_completion(false)),
        }
    }

    fn run_as_historic_with_completion(&self, was_completed: bool) -> TimedRun<ShortDescriptor> {
        TimedRun {
            category_locator: self.metadata.short.clone(),
            was_completed,
            date: chrono::Utc::now(),
            timing: self.run.timing_as_historic(),
        }
    }
}

/// The session exposes its underlying run as a split set.
impl Set for Session {
    fn reset(&mut self) {
        for o in &self.observers {
            o.on_reset(&self)
        }
        self.run.reset()
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
