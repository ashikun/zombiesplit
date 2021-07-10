//! Parts of the model related to comparisons.

pub mod pace;

use crate::model::Time;
pub use pace::Pace;

use self::pace::PacedTime;

use super::game::category::ShortDescriptor;

/// A comparison set.
pub struct Comparison {
    pub splits: Vec<Split>,
}

impl Comparison {
    /// Compares `run_time` against the cumulative time at `split`.
    #[must_use]
    pub fn run_paced_time_at(&self, split: usize, run_time: Time) -> PacedTime {
        self.splits
            .get(split)
            .map_or(PacedTime::inconclusive(run_time), |x| {
                x.run_paced_time(run_time)
            })
    }

    /// Compares `split_time` against the split data at `split`.
    #[must_use]
    pub fn split_paced_time_at(&self, split: usize, split_time: Time) -> PacedTime {
        self.splits
            .get(split)
            .map_or(PacedTime::inconclusive(split_time), |x| {
                x.split_paced_time(split_time)
            })
    }
}

/// Split comparisons.
pub struct Split {
    /// The personal best for this split, if any.
    ///
    /// Any splits that compare quicker than this time get the `PersonalBest`
    /// pace.
    pub split: Option<Time>,
    /// The time for this split in the comparison run, if any.
    ///
    /// This feeds into the split-by-split part of a split's in-run pacing.
    pub in_run: Option<Time>,
    /// The cumulative time for this split in the comparison run, if any.
    ///
    /// This feeds into the run-so-far part of a split's in-run pacing.
    pub in_run_cumulative: Option<Time>,
}

impl Split {
    /// Compares `run_time` against the cumulative time at this split.
    #[must_use]
    pub fn run_paced_time(&self, run_time: Time) -> PacedTime {
        PacedTime::of_comparison(run_time, self.in_run_cumulative)
    }

    /// Compares `split_time` against the split data for this comparison.
    #[must_use]
    pub fn split_paced_time(&self, split_time: Time) -> PacedTime {
        if self.is_personal_best(split_time) {
            PacedTime::personal_best(split_time)
        } else {
            PacedTime::of_comparison(split_time, self.in_run)
        }
    }

    /// Checks whether `split time` is a new personal best.
    fn is_personal_best(&self, split_time: Time) -> bool {
        self.split.map_or(false, |pb| split_time < pb)
    }
}

/// Trait of objects that can provide comparisons.
pub trait Provider {
    /// Gets the current comparison for a game-category.
    fn comparison(&mut self, short: &ShortDescriptor) -> Option<Comparison>;
}

/// A provider that never provides comparisons.
pub struct NullProvider;

impl Provider for NullProvider {
    fn comparison(&mut self, _short: &ShortDescriptor) -> Option<Comparison> {
        None
    }
}
