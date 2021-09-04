//! Models relating to runs.

use crate::model::{game::category::AttemptInfo, history};

use super::{
    super::time::Time,
    split::{Set, Split},
};

/// An in-progress run.
pub struct Run {
    /// Attempt information for this run.
    pub attempt: AttemptInfo,
    /// The split data for this run.
    pub splits: Vec<Split>,
}

/// A run is a set of splits.
impl Set for Run {
    fn reset(&mut self) {
        self.increment_attempt();
        self.splits.iter_mut().for_each(Split::clear);
    }

    fn push_to(&mut self, split: usize, time: Time) {
        if let Some(ref mut s) = self.splits.get_mut(split) {
            s.push(time);
        }
    }

    fn pop_from(&mut self, split: usize) -> Option<Time> {
        self.splits.get_mut(split).and_then(Split::pop)
    }

    fn clear_at(&mut self, split: usize) {
        if let Some(s) = self.splits.get_mut(split) {
            s.clear();
        }
    }
}

impl Run {
    /// Gets the cumulative time at the split `split`.
    #[must_use]
    pub fn cumulative_at(&self, split: usize) -> Time {
        // TODO(@MattWindsor91): make this more efficient (O(1) access).
        self.splits
            .iter()
            .take(split + 1)
            .map(Split::summed_time)
            .sum()
    }

    /// Gets the total time of the split `split`.
    #[must_use]
    pub fn time_at(&self, split: usize) -> Time {
        self.splits
            .get(split)
            .map_or(Time::default(), Split::summed_time)
    }

    /// Gets the number of splits in this run.
    #[must_use]
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }

    fn increment_attempt(&mut self) {
        if let Some(is_completed) = self.status().to_completeness() {
            self.attempt.increment(is_completed);
        }
    }

    /// Gets the current status of the run, based on how many splits have been
    /// filled in.
    #[must_use]
    pub fn status(&self) -> Status {
        match self.num_filled_splits() {
            0 => Status::NotStarted,
            x if x == self.num_splits() => Status::Complete,
            _ => Status::Incomplete,
        }
    }

    fn num_filled_splits(&self) -> usize {
        self.splits.iter().filter(|x| 0 < x.num_times()).count()
    }

    /// Gets a history summary of the timing for this run.
    #[must_use]
    pub fn timing_as_historic(&self) -> history::timing::Full {
        history::timing::Full {
            times: self
                .splits
                .iter()
                .map(|s| (s.info.short, s.all_times()))
                .collect(),
        }
    }
}

/// The status of the run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// The run hasn't been started yet.
    NotStarted,
    /// The run appears to be incomplete.
    Incomplete,
    /// The run appears to be complete.
    Complete,
}

impl Status {
    /// Gets whether this run has been started and, if so, whether it is
    /// completed.
    #[must_use]
    pub fn to_completeness(self) -> Option<bool> {
        match self {
            Self::NotStarted => None,
            Self::Incomplete => Some(false),
            Self::Complete => Some(true),
        }
    }
}
