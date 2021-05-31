//! Models relating to an in-progress run.

use super::{
    split::{Comparison, Pace, Split},
    time::Time,
};

/// An in-progress run.
pub struct Run {
    pub metadata: Metadata,
    /// The attempt number of this run.
    pub attempt: usize,
    pub splits: Vec<Split>,
    pub comparisons: Vec<Comparison>,
}

impl Run {
    /// Wipes all times for this run.
    pub fn reset(&mut self) {
        self.attempt += 1;
        self.splits.iter_mut().for_each(Split::clear)
    }

    /// Pushes the time `time` onto the split at `split`, if it exists.
    pub fn push_to(&mut self, split: usize, time: Time) {
        if let Some(ref mut s) = self.splits.get_mut(split) {
            s.push(time)
        }
    }

    /// Pops a time from the split at `split`, if it exists.
    pub fn pop_from(&mut self, split: usize) -> Option<Time> {
        self.splits.get_mut(split).and_then(Split::pop)
    }

    /// Removes all times from the split at `split`, if it exists.
    pub fn reset_at(&mut self, split: usize) {
        if let Some(s) = self.splits.get_mut(split) {
            s.clear()
        }
    }

    /// Gets the total time across all splits.
    #[must_use]
    pub fn total_time(&self) -> Time {
        self.splits.iter().map(Split::summed_time).sum()
    }

    /// Gets the pace for the split at `split`.
    #[must_use]
    pub fn pace_at(&self, split: usize) -> Pace {
        // TODO(@MattWindsor91): return 2D run/split pacing
        // TODO(@MattWindsor91): compare split cumulatives
        self.splits.get(split).map_or(Pace::default(), |s| {
            self.comparisons
                .get(split)
                .map_or(Pace::default(), |c| c.pace(s.summed_time()))
        })
    }
}

/// Metadata in a run.
pub struct Metadata {
    /// The name of the game.
    pub game: String,
    /// The name of the category.
    pub category: String,
}
