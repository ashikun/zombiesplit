//! Models relating to in-progress runs.

use super::split::Set;
use crate::model::{game::category, history};
use chrono::{DateTime, Utc};

/// An in-progress run.
///
/// The representation of an in-progress run consists of a split set and some
/// metadata about the run itself (the game, the category, how many attempts
/// have been made, etc).
#[derive(Debug, Clone)]
pub struct Attempt {
    /// Metadata for the game/category currently being run.
    pub category: category::Info,
    // TODO(@MattWindsor91): make this an ADT to prevent resetting of splits
    // without incrementing of attempt information.
    /// Attempt information for this run.
    pub info: category::AttemptInfo,
    /// The split data for this run.
    pub splits: Set,
}

impl Attempt {
    /// Resets this run and all splits inside it, incrementing the attempt if necessary.
    pub fn reset(&mut self, dest: super::action::OldDestination) {
        if matches!(dest, super::action::OldDestination::Save) {
            self.increment_attempt();
        }
        self.splits.reset();
    }

    fn increment_attempt(&mut self) {
        if let Some(is_completed) = self.status().to_completeness() {
            self.info.increment(is_completed);
        }
    }

    /// Gets the current status of the run, based on how many splits have been
    /// filled in.
    #[must_use]
    pub fn status(&self) -> Status {
        match self.num_filled_splits() {
            0 => Status::NotStarted,
            x if x == self.splits.len() => Status::Complete,
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

    /// Converts this run, if any, to a historic run on `date`.
    ///
    /// Returns `None` if the run has no timing on any splits (in which case,
    /// recording the historic run would be pointless).
    #[must_use]
    pub fn as_historic(
        &self,
        date: DateTime<Utc>,
    ) -> Option<history::run::FullyTimed<category::ShortDescriptor>> {
        self.status()
            .to_completeness()
            .map(|c| self.as_historic_with_completion(c, date))
    }

    fn as_historic_with_completion(
        &self,
        was_completed: bool,
        date: DateTime<Utc>,
    ) -> history::run::FullyTimed<category::ShortDescriptor> {
        history::run::FullyTimed {
            category_locator: self.category.short,
            was_completed,
            date,
            timing: self.timing_as_historic(),
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
