//! Structs and functions for pace computation.

use crate::model::Time;

/// Possible paces for a split or run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pace {
    /// There is no pacing time.
    Inconclusive,
    /// This split, or run, is behind its comparison.
    Behind,
    /// This split, or run, is ahead its comparison.
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
