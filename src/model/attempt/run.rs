//! Models relating to runs.

use super::split::Set;
use crate::model::{game::category::AttemptInfo, history, short};

/// An in-progress run.
pub struct Run {
    // TODO(@MattWindsor91): make this an ADT to prevent resetting of splits
    // without incrementing of attempt information.
    /// Attempt information for this run.
    pub attempt: AttemptInfo,
    /// The split data for this run.
    pub splits: Set,
}

impl Run {
    /// Resets the run, incrementing its attempt counter.
    pub fn reset(&mut self) {
        self.increment_attempt();
        self.splits.reset();
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

    /// Gets the number of splits in this run.
    #[must_use]
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }

    /// Gets the position of the split with short name `short`.
    #[must_use]
    pub fn position_of(&self, short: impl Into<short::Name>) -> Option<usize> {
        self.splits.position_of(short)
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
