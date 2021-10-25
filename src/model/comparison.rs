//! Parts of the model related to comparisons.

pub mod pace;

pub use pace::{Pace, PacedTime};

use super::{aggregate, short, Time};

/// A comparison set.
#[derive(Clone, Debug, Default)]
pub struct Comparison(pub short::Map<Split>);

impl Comparison {
    /// Gets a pace note for the split with short name `split`, which has just
    /// posted an aggregate time pair of `against`.
    #[must_use]
    pub fn pace(&self, split: short::Name, against: aggregate::Set) -> pace::SplitInRun {
        self.0
            .get(&split)
            .map_or(pace::SplitInRun::Inconclusive, |x| x.pace(against))
    }

    /// Gets the aggregate times for the split with short name `split`, if
    /// available.
    #[must_use]
    pub fn aggregate_for(&self, split: short::Name) -> Option<&aggregate::Set> {
        self.0.get(&split).and_then(|x| x.in_run.as_ref())
    }
}

/// A [Comparison] can be created using named split comparisons.
impl FromIterator<(short::Name, Split)> for Comparison {
    fn from_iter<T: IntoIterator<Item = (short::Name, Split)>>(iter: T) -> Self {
        Comparison(iter.into_iter().collect())
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
    /// The personal best for this split, if any.
    ///
    /// Any splits that compare quicker than this time get the `PersonalBest`
    /// pace.
    pub split_pb: Option<Time>,
    /// Timing information for this split in the comparison run, if any.
    pub in_run: Option<aggregate::Set>,
}

impl Split {
    /// Gets a pace note for this split, which has just posted an aggregate time
    /// pair of `against`.
    #[must_use]
    pub fn pace(&self, against: aggregate::Set) -> pace::SplitInRun {
        pace::SplitInRun::new(
            self.split_pace(against.split),
            self.cumulative_pace(against.cumulative),
        )
    }

    /// Gets the aggregate time of scope `scope` for this split in the run
    /// against which we are comparing.
    #[must_use]
    fn aggregate_in_run(&self, scope: aggregate::Scope) -> Option<Time> {
        self.in_run.map(|x| x[scope])
    }

    /// Compares `time` against the cumulative time at this split.
    #[must_use]
    pub fn cumulative_pace(&self, time: Time) -> Pace {
        Pace::of_comparison(time, self.aggregate_in_run(aggregate::Scope::Cumulative))
    }

    /// Compares `split_time` against the split data for this comparison.
    #[must_use]
    pub fn split_pace(&self, time: Time) -> Pace {
        if self.is_personal_best(time) {
            Pace::PersonalBest
        } else {
            Pace::of_comparison(time, self.aggregate_in_run(aggregate::Scope::Split))
        }
    }

    /// Checks whether `split time` is a new personal best.
    fn is_personal_best(&self, split_time: Time) -> bool {
        self.split_pb.map_or(false, |pb| split_time < pb)
    }
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
