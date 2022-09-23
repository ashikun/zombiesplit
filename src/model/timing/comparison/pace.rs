//! Structs and functions for pace computation.

use crate::model::timing::time::human;
use serde::{Deserialize, Serialize};

/// Possible paces for a split or run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Pace {
    /// There is comparison for this time.
    Inconclusive,
    /// Time is behind its comparison.
    Behind,
    /// Time is ahead (or breaking even on) its comparison.
    Ahead,
    /// Time is a personal best ('gold split').
    PersonalBest,
}

/// The default pace is inconclusive.
impl Default for Pace {
    fn default() -> Self {
        Self::Inconclusive
    }
}

impl Pace {
    /// Calculates the pace by comparing `time` to `compared_to`.
    ///
    /// ```
    /// use zombiesplit::model::timing::{comparison::Pace, time::human};
    ///
    /// let t1 = human::Time::seconds(16).unwrap();
    /// let t2 = human::Time::seconds(80).unwrap();
    ///
    /// assert_eq!(Pace::Ahead, Pace::of_comparison(t1, t2));
    /// assert_eq!(Pace::Behind, Pace::of_comparison(t2, t1));
    ///
    /// // When there is no time difference, the result is `Ahead`.
    /// assert_eq!(Pace::Ahead, Pace::of_comparison(t1, t1));
    /// ```
    #[must_use]
    pub fn of_comparison(time: human::Time, compared_to: human::Time) -> Self {
        if time <= compared_to {
            Self::Ahead
        } else {
            Self::Behind
        }
    }
}

/// A pair of a time and its pace against comparison.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PacedTime {
    /// The pace.
    pub pace: Pace,
    /// The time to which `pace` applies.
    pub time: human::Time,
}

impl PacedTime {
    /// Shorthand for wrapping a time in a `Pace::Inconclusive` paced time.
    #[must_use]
    pub fn inconclusive(time: human::Time) -> Self {
        Self {
            pace: Pace::Inconclusive,
            time,
        }
    }

    /// Shorthand for wrapping a time in a `Pace::PersonalBest` paced time.
    #[must_use]
    pub fn personal_best(time: human::Time) -> Self {
        Self {
            pace: Pace::PersonalBest,
            time,
        }
    }

    /// Calculates the pace by comparing `time` to `compared_to`, if it exists.
    #[must_use]
    pub fn of_comparison(time: human::Time, compared_to: human::Time) -> Self {
        Self {
            pace: Pace::of_comparison(time, compared_to),
            time,
        }
    }
}

/// Combined pace note for a split in the context of a run in progress.
///
/// These note the pace of a run, as well as how the current split has affected
/// it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SplitInRun {
    /// There is no pacing time.
    Inconclusive,
    /// The split is a personal best ('gold split').
    SplitPersonalBest,
    /// The run is behind, and we lost time on the given split.
    BehindAndLosing,
    /// The run is behind, but we gained time (or broke even) on the given split.
    BehindAndGaining,
    /// The run is ahead, but we lost time on the given split.
    AheadAndLosing,
    /// The run is ahead, and we gained time (or broke even) on the given split.
    AheadAndGaining,
}

/// The default split-in-run pace is inconclusive.
impl Default for SplitInRun {
    fn default() -> Self {
        Self::Inconclusive
    }
}

impl SplitInRun {
    /// Constructs a split-in-run pace note from a `split` and `run_so_far` pace.
    ///
    /// ```
    /// use zombiesplit::model::timing::comparison::pace;
    ///
    /// assert_eq!(
    ///     pace::SplitInRun::Inconclusive,
    ///     pace::SplitInRun::new(pace::Pace::Inconclusive, pace::Pace::Ahead));
    /// assert_eq!(
    ///     pace::SplitInRun::Inconclusive,
    ///     pace::SplitInRun::new(pace::Pace::Behind, pace::Pace::Inconclusive));
    /// assert_eq!(
    ///     pace::SplitInRun::SplitPersonalBest,
    ///     pace::SplitInRun::new(pace::Pace::PersonalBest, pace::Pace::Behind));
    /// assert_eq!(
    ///     pace::SplitInRun::BehindAndGaining,
    ///     pace::SplitInRun::new(pace::Pace::Ahead, pace::Pace::Behind));
    /// assert_eq!(
    ///     pace::SplitInRun::AheadAndLosing,
    ///     pace::SplitInRun::new(pace::Pace::Behind, pace::Pace::Ahead));
    /// ```
    #[must_use]
    pub fn new(split: Pace, run_so_far: Pace) -> Self {
        match (split, run_so_far) {
            (Pace::Inconclusive, _) | (_, Pace::Inconclusive) => Self::Inconclusive,
            (Pace::PersonalBest, _) => Self::SplitPersonalBest,
            (Pace::Behind, Pace::Behind) => Self::BehindAndLosing,
            (Pace::Ahead, Pace::Behind) => Self::BehindAndGaining,
            (Pace::Behind, Pace::Ahead | Pace::PersonalBest) => Self::AheadAndLosing,
            (Pace::Ahead, Pace::Ahead | Pace::PersonalBest) => Self::AheadAndGaining,
        }
    }

    /// Extracts the overall pace note from a split-in-run pace.
    ///
    /// ```
    /// use zombiesplit::model::timing::comparison::pace;
    ///
    /// let p = pace::SplitInRun::new(pace::Pace::Ahead, pace::Pace::Behind);
    /// assert_eq!(pace::Pace::Behind, p.overall());
    /// ```
    #[must_use]
    pub fn overall(self) -> Pace {
        match self {
            SplitInRun::Inconclusive => Pace::Inconclusive,
            SplitInRun::SplitPersonalBest | SplitInRun::AheadAndGaining | Self::AheadAndLosing => {
                Pace::Ahead
            }
            SplitInRun::BehindAndLosing | SplitInRun::BehindAndGaining => Pace::Behind,
        }
    }
}
