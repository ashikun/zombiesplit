//! The [Session] type and related code.

use super::{
    comparison::{pace, Comparison},
    run::Run,
    split::Set,
    time::Time,
};

/// Model data for a running session.
pub struct Session {
    /// Metadata for the game/category currently being run.
    pub metadata: Metadata,
    /// The current run.
    run: Run,
    /// Comparison data for the game/category currently being run.
    comparisons: Comparison,
}

impl Session {
    /// Starts a new session with a given set of metadata and starting run.
    #[must_use]
    pub fn new(metadata: Metadata, run: Run) -> Self {
        Self {
            metadata,
            run,
            comparisons: Comparison { splits: vec![] },
        }
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
}

/// The session exposes its underlying run as a split set.
impl Set for Session {
    fn reset(&mut self) {
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

/// Metadata in a run.
pub struct Metadata {
    /// The numeric ID of the category in the database.
    pub category_id: i64,
    /// The name of the game.
    pub game: String,
    /// The name of the category.
    pub category: String,
}
