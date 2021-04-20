//! Models relating to an in-progress run.

use std::ops::Add;

/// An in-progress run.
pub struct Run {
    pub splits: Vec<Split>,
}

/// A split in a run.
pub struct Split {
    pub name: String,
    // The entered times.
    pub times: Vec<super::time::Time>,
}

impl Split {
    /// Calculates the summed time of the split.
    pub fn summed_time(&self) -> Option<super::time::Time> {
        self.times.iter().copied().reduce(super::time::Time::add)
    }
}
