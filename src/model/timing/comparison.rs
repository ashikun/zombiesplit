//! Parts of the model related to comparisons.

pub mod delta;
pub mod pace;
pub mod provider;
pub mod run;

pub use delta::Delta;
pub use pace::{Pace, PacedTime};
pub use provider::Provider;
pub use run::Run;

use super::{super::short, aggregate, Time};

/// Comparison data, containing information about split and run personal bests (PBs).
///
/// A comparison struct contains both per-split and aggregated times.  There is no inherent checking
/// that the former and latter agree, as some comparison providers may have ways of calculating the
/// latter that don't involve
#[derive(Clone, Debug, Default)]
pub struct Comparison {
    /// Split comparisons.
    pub splits: short::Map<Split>,
    /// Precomputed run data.
    pub run: Run,
}

impl Comparison {
    /// Gets a delta for the split with short name `split`, which has just
    /// posted an aggregate time pair of `against`.
    ///
    /// # Errors
    ///
    /// Fails if the delta cannot be represented as a time.
    pub fn delta(
        &self,
        split: short::Name,
        against: aggregate::Set,
    ) -> Result<delta::Split, super::time::Error> {
        self.splits
            .get(&split)
            .map_or(Ok(delta::Split::default()), |x| x.delta(against))
    }

    /// Gets the aggregate times for the split with short name `split`, if
    /// available.
    #[must_use]
    pub fn aggregate_for(&self, split: short::Name) -> Option<&aggregate::Set> {
        self.splits.get(&split).map(|x| &x.in_pb_run)
    }
}

/// A [Comparison] can be turned back into an iterator over split name-comparison pairs.
impl IntoIterator for Comparison {
    type Item = (short::Name, Split);
    type IntoIter = <short::Map<Split> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.splits.into_iter()
    }
}

/// Split comparisons.
///
/// A split comparison contains (for now) up to two pieces:
///
/// - a 'personal best' for the split across all runs stored in the database
///   (used for calculating so-called 'gold splits');
/// - a set of aggregates that represent the important times logged for this
///   split on the comparison run (right now, there is only one comparison run,
///   the PB).
#[derive(Clone, Copy, Debug, Default)]
pub struct Split {
    /// The personal best for this split.
    ///
    /// Any splits that compare quicker than this time get the `PersonalBest`
    /// pace.
    pub split_pb: Time,
    /// Timing information for this split in the comparison run.
    pub in_pb_run: aggregate::Set,
}

impl Split {
    /// Gets delta information for this split, which has just posted an aggregate time
    /// pair of `against`.
    ///
    /// # Errors
    ///
    /// Fails if the delta cannot be represented as a time.
    pub fn delta(&self, against: aggregate::Set) -> Result<delta::Split, super::time::Error> {
        let mut split = self.delta_against_aggregate(against, aggregate::Scope::Split)?;

        // TODO: separate PB from this consideration entirely.
        if self.is_personal_best(against.split) {
            split.pace = Pace::PersonalBest;
        }

        let run = self.delta_against_aggregate(against, aggregate::Scope::Cumulative)?;

        Ok(delta::Split::new(split, run))
    }

    /// Gets a delta for the aggregate time of scope `scope` between `against` and this comparison.
    ///
    /// # Errors
    ///
    /// Fails if the delta cannot be represented as a time.
    fn delta_against_aggregate(
        &self,
        against: aggregate::Set,
        scope: aggregate::Scope,
    ) -> Result<delta::Delta, super::time::Error> {
        delta::Delta::of_comparison(against[scope], self.in_pb_run[scope])
    }

    /// Checks whether `split time` is a new personal best.
    fn is_personal_best(&self, split_time: Time) -> bool {
        split_time < self.split_pb
    }
}
