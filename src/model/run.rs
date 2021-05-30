//! Models relating to an in-progress run.

use super::time::Time;

/// An in-progress run.
pub struct Run {
    pub metadata: Metadata,
    /// The attempt number of this run.
    pub attempt: usize,
    pub splits: Vec<Split>,
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
}

/// Metadata in a run.
pub struct Metadata {
    /// The name of the game.
    pub game: String,
    /// The name of the category.
    pub category: String,
}

/// A split in a run.
pub struct Split {
    /// The name of the split.
    pub name: String,
    /// The entered times.
    /// Invariant: none of the times are zero.
    times: Vec<super::time::Time>,
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

    /// Gets whether this split has times registered.
    #[must_use]
    pub fn has_times(&self) -> bool {
        !self.times.is_empty()
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
