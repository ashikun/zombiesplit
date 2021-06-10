//! Splits and related items.

use super::time::Time;

/// A split in a run.
pub struct Split {
    /// The name of the split.
    pub name: String,
    /// The entered times.
    /// Invariant: none of the times are zero.
    times: Vec<Time>,
}

impl Split {
    /// Creates a new split with the given name and an empty time.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            times: Vec::new(),
        }
    }

    /// Calculates the summed time of the split.
    #[must_use]
    pub fn summed_time(&self) -> Time {
        self.times.iter().copied().sum()
    }

    /// Gets the number of times logged for this split.
    #[must_use]
    pub fn num_times(&self) -> usize {
        self.times.len()
    }

    /// Pushes a time onto this split.
    ///
    /// If the time is zero, it will not be added.
    pub fn push(&mut self, time: Time) {
        if !time.is_zero() {
            self.times.push(time)
        }
    }

    /// Tries to pop the most recently added time off this split.
    #[must_use]
    pub fn pop(&mut self) -> Option<Time> {
        self.times.pop()
    }

    /// Removes all times from this split.
    pub fn clear(&mut self) {
        self.times.clear()
    }
}

/// Trait for things that contain splits.
pub trait Set {
    /// Gets the name of the split at `split`, if it exists.
    fn name_at(&self, split: usize) -> &str;

    /// Wipes all data for all splits, incrementing any attempt counter.
    fn reset(&mut self);

    /// Removes all times from the split at `split`, if it exists.
    fn clear_at(&mut self, split: usize);

    /// Pushes the time `time` onto the split at `split`, if it exists.
    fn push_to(&mut self, split: usize, time: Time);

    /// Pops a time from the split at `split`, if it exists.
    fn pop_from(&mut self, split: usize) -> Option<Time>;

    /// Gets the total time up to and including `split`.
    #[must_use]
    fn total_at(&self, split: usize) -> Time;

    /// Gets the number of times logged for the split at `split`.
    #[must_use]
    fn num_times_at(&self, split: usize) -> usize;

    /// Gets the summed time logged for the split at `split`.
    #[must_use]
    fn time_at(&self, split: usize) -> Time;

    /// Gets the number of splits in this set.
    #[must_use]
    fn num_splits(&self) -> usize;
}
