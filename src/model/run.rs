//! Models relating to an in-progress run.

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
    #[must_use]
    pub fn summed_time(&self) -> super::time::Time {
        self.times.iter().copied().sum()
    }
}
