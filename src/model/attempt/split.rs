//! Splits and related items.

pub mod set;

use super::super::{game, time::Time};

pub use set::{Locator, Set};

/// A split in a run attempt.
#[derive(Debug)]
pub struct Split {
    /// The game/category split information for this split.
    ///
    /// This contains the split's shortname, its default display name, and other
    /// such information.
    pub info: game::Split,
    /// The entered times.
    /// Invariant: none of the times are zero.
    times: Vec<Time>,
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

    /// Clones a copy of the times for this split.
    #[must_use]
    pub fn all_times(&self) -> Vec<Time> {
        self.times.clone()
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
            self.times.push(time);
        }
    }

    /// Tries to pop the most recently added time off this split.
    #[must_use]
    pub fn pop(&mut self) -> Option<Time> {
        self.times.pop()
    }

    /// Removes all times from this split.
    pub fn clear(&mut self) {
        self.times.clear();
    }
}
