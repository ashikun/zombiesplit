//! Structs and functions for pace computation.

use crate::model::Time;

/// Possible paces for a split or run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pace {
    /// There is no pacing time.
    Inconclusive,
    /// This split, or run, is behind its comparison.
    Behind,
    /// This split, or run, is ahead (or breaking even on) its comparison.
    Ahead,
    /// This split, or run, is a personal best ('gold split').
    PersonalBest,
}

/// The default pace is inconclusive.
impl Default for Pace {
    fn default() -> Self {
        Self::Inconclusive
    }
}

impl Pace {
    /// Calculates the pace by comparing `time` to `compared_to`, if it exists.
    #[must_use]
    pub fn of_comparison(time: Time, compared_to: Option<Time>) -> Self {
        compared_to.map_or(Self::default(), |cmp| {
            if time <= cmp {
                Self::Ahead
            } else {
                Self::Behind
            }
        })
    }
}

/// A pair of a time and its pace against comparison.
#[derive(Clone, Copy, Debug, Default)]
pub struct PacedTime {
    /// The pace.
    pub pace: Pace,
    /// The time to which `pace` applies.
    pub time: Time,
}

impl PacedTime {
    /// Shorthand for wrapping a time in a `Pace::Inconclusive` paced time.
    #[must_use]
    pub fn inconclusive(time: Time) -> Self {
        Self {
            pace: Pace::Inconclusive,
            time,
        }
    }

    /// Shorthand for wrapping a time in a `Pace::PersonalBest` paced time.
    #[must_use]
    pub fn personal_best(time: Time) -> Self {
        Self {
            pace: Pace::PersonalBest,
            time,
        }
    }

    /// Calculates the pace by comparing `time` to `compared_to`, if it exists.
    #[must_use]
    pub fn of_comparison(time: Time, compared_to: Option<Time>) -> Self {
        Self {
            pace: Pace::of_comparison(time, compared_to),
            time,
        }
    }
}

/// A pair of split pace and run-so-far pace.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pair {
    /// The split pace.
    pub split: PacedTime,
    /// The run-so-far pace.
    pub run_so_far: PacedTime,
}

impl Pair {
    /// Gets the combined split-in-run pace note for this pair.
    #[must_use]
    pub fn split_in_run_pace(&self) -> SplitInRun {
        SplitInRun::new(self.split.pace, self.run_so_far.pace)
    }
}

/// Combined pace note for a split in the context of a run in progress.
///
/// These note the pace of a run, as well as how the current split has affected
/// it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

impl SplitInRun {
    /// Constructs a split-in-run pace note from a `split` and `run_so_far` pace.
    ///
    /// ```
    /// use zombiesplit::model::comparison::pace;
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
}
