//! Models relating to an in-progress run.

use super::{
    pace,
    split::{Comparison, Split},
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

    /// Gets the total time up to and including `split`.
    #[must_use]
    pub fn total_at(&self, split: usize) -> Time {
        self.splits
            .iter()
            .take(split + 1)
            .map(Split::summed_time)
            .sum()
    }

    /// Gets the number of times logged for the split at `split`.
    #[must_use]
    pub fn num_times_at(&self, split: usize) -> usize {
        self.splits.get(split).map_or(0, Split::num_times)
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

    fn run_paced_time_at(&self, split: usize) -> pace::PacedTime {
        let time = self.total_at(split);
        // TODO(@MattWindsor91): do this.
        let pace = pace::Pace::default();
        pace::PacedTime { pace, time }
    }

    fn split_paced_time_at(&self, split: usize) -> pace::PacedTime {
        // TODO(@MattWindsor91): compare split cumulatives
        self.splits
            .get(split)
            .map_or(pace::PacedTime::default(), |s| {
                let time = s.summed_time();
                pace::PacedTime {
                    pace: self.split_pace_at(split, time),
                    time,
                }
            })
    }

    fn split_pace_at(&self, split: usize, time: Time) -> pace::Pace {
        self.comparisons
            .get(split)
            .map_or(pace::Pace::default(), |c| c.pace(time))
    }
}

/// Metadata in a run.
pub struct Metadata {
    /// The name of the game.
    pub game: String,
    /// The name of the category.
    pub category: String,
}
