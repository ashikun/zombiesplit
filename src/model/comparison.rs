//! Parts of the model related to comparisons.

pub mod pace;

pub use pace::{Pace, PacedTime};

use super::{short::LinkedMap, Time};

/// A comparison set.
#[derive(Clone, Debug, Default)]
pub struct Comparison {
    pub splits: LinkedMap<Split>,
}

impl Comparison {
    /// Compares `run_time` against the cumulative time at `split`.
    #[must_use]
    pub fn run_paced_time(&self, split: usize, run_time: Time) -> PacedTime {
        self.split(split)
            .map_or(PacedTime::inconclusive(run_time), |x| {
                x.run_paced_time(run_time)
            })
    }

    /// Compares `split_time` against the split data at `split`.
    #[must_use]
    pub fn split_paced_time(&self, split: usize, split_time: Time) -> PacedTime {
        self.split(split)
            .map_or(PacedTime::inconclusive(split_time), |x| {
                x.split_paced_time(split_time)
            })
    }

    /// Gets the comparison time for `split`.
    #[must_use]
    pub fn split_comparison_time(&self, split: usize) -> Option<Time> {
        self.split(split).and_then(|f| f.in_run.map(|x| x.time))
    }

    /// Gets the comparison for the split at `index`.
    #[must_use]
    pub fn split(&self, index: usize) -> Option<&Split> {
        // TODO(@MattWindsor91): this is O(n), don't.
        self.splits.values().nth(index)
    }
}

/// Split comparisons.
#[derive(Clone, Copy, Debug, Default)]
pub struct Split {
    // TODO(@MattWindsor91): this is basically an aggregate pair.
    /// The personal best for this split, if any.
    ///
    /// Any splits that compare quicker than this time get the `PersonalBest`
    /// pace.
    pub split: Option<Time>,
    /// Timing information for this split in the comparison run, if any.
    pub in_run: Option<InRun>,
}

impl Split {
    /// Compares `run_time` against the cumulative time at this split.
    #[must_use]
    pub fn run_paced_time(&self, run_time: Time) -> PacedTime {
        PacedTime::of_comparison(run_time, self.in_run.map(|x| x.cumulative))
    }

    /// Compares `split_time` against the split data for this comparison.
    #[must_use]
    pub fn split_paced_time(&self, split_time: Time) -> PacedTime {
        if self.is_personal_best(split_time) {
            PacedTime::personal_best(split_time)
        } else {
            PacedTime::of_comparison(split_time, self.in_run.map(|x| x.time))
        }
    }

    /// Checks whether `split time` is a new personal best.
    fn is_personal_best(&self, split_time: Time) -> bool {
        self.split.map_or(false, |pb| split_time < pb)
    }
}

/// Holds information about a split's comparisons within a particular run.
#[derive(Clone, Copy, Debug, Default)]
pub struct InRun {
    /// The time for this split in the comparison run, if any.
    ///
    /// This feeds into the split-by-split part of a split's in-run pacing.
    pub time: Time,
    /// The cumulative time for this split in the comparison run, if any.
    ///
    /// This feeds into the run-so-far part of a split's in-run pacing.
    pub cumulative: Time,
}

/// Trait of objects that can provide comparisons.
pub trait Provider {
    /// Gets the current comparison for a game-category.
    fn comparison(&mut self) -> Option<Comparison>;
}

/// A provider that never provides comparisons.
pub struct NullProvider;

impl Provider for NullProvider {
    fn comparison(&mut self) -> Option<Comparison> {
        None
    }
}
