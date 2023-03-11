//! Splits and related items.

pub use set::{Locator, Set};

use super::super::{game, timing::time};

pub mod set;

/// A split in a run attempt.
#[derive(Debug, Clone)]
pub struct Split {
    /// The game/category split information for this split.
    ///
    /// This contains the split's shortname, its default display name, and other
    /// such information.
    pub info: game::Split,
    /// The entered times.
    pub times: Vec<time::Time>,
}

impl Split {
    /// Creates a new split with the given metadata and an empty time.
    #[must_use]
    pub fn new(info: game::Split) -> Self {
        Self {
            info,
            times: Vec::new(),
        }
    }

    /// Borrows this split's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.info.name
    }

    /// Gets the total time for this split.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::{session::split::Split, game, timing::time};
    ///
    /// let mut s = Split::new(game::Split::new("pp1", "Palmtree Panic 1"));
    /// s.push(time::Time::from_millis(9));
    /// s.push(time::Time::from_millis(10));
    /// assert_eq!(time::Time::from_millis(19), s.total_time());
    /// ```
    #[must_use]
    pub fn total_time(&self) -> time::Time {
        self.times.iter().copied().sum()
    }

    /// Clones a copy of the times for this split.
    #[must_use]
    pub fn all_times(&self) -> Vec<time::Time> {
        self.times.clone()
    }

    /// Gets the number of times logged for this split.
    #[must_use]
    pub fn num_times(&self) -> usize {
        self.times.len()
    }

    /// Pushes a time onto this split.
    pub fn push(&mut self, time: time::Time) {
        self.times.push(time);
    }

    /// Tries to pop the most recently added time off this split.
    #[must_use]
    pub fn pop(&mut self) -> Option<time::Time> {
        self.times.pop()
    }

    /// Removes all times from this split.
    pub fn clear(&mut self) {
        self.times.clear();
    }
}
