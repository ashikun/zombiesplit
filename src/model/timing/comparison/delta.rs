//! Deltas between times.
//!
//! These records record both pace information as well as an absolute delta between attempt and
//! comparison times.  (We do not use the subtraction delta because this would require storing
//! negative times, which we don't support, and the combination of absolute difference and
//! pace is unambiguous.)

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{super::time, pace};

/// A single-time time difference.
///
/// Usually, these will represent run-wide cumulative deltas, but they are also used to construct
/// [Split]s.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    /// The pace.
    pub pace: pace::Pace,
    /// The absolute amount by which the times differ.
    pub abs_delta: time::Time,
}

impl FromStr for Delta {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cs = s.chars();
        let pace = delta_sign(cs.next().unwrap_or('+'))?;
        let abs_delta = cs.as_str().parse()?;
        Ok(Self { pace, abs_delta })
    }
}

/// Converts a prefix sign into a pace (`+` means behind, `-` means ahead, etc.)
///
/// Note that the sign is from the
///
/// # Errors
///
/// Fails if `sign` does not represent a pace type.
fn delta_sign(sign: char) -> Result<pace::Pace> {
    Ok(match sign {
        '-' => pace::Pace::Ahead,
        '+' => pace::Pace::Behind,
        '*' => pace::Pace::PersonalBest,
        '?' => pace::Pace::Inconclusive,
        _ => Err(Error::DeltaSign { sign })?,
    })
}

/// An error that occurs when parsing a delta.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("time failed parsing")]
    TimeParse(#[from] super::super::time::Error),
    #[error("bad sign '{sign}' for delta")]
    DeltaSign { sign: char },
}

/// Shorthand for parse results.
pub type Result<T> = std::result::Result<T, Error>;

impl Delta {
    /// Calculates the delta by comparing `time` to `compared_to`, if it exists.
    ///
    /// # Errors
    ///
    /// Fails if we can't capture the delta as a [Time].
    pub fn of_comparison(time: time::Time, compared_to: time::Time) -> time::error::Result<Self> {
        Ok(Self {
            pace: pace::Pace::of_comparison(time, compared_to),
            abs_delta: time::Time::try_from(u32::abs_diff(
                u32::from(time),
                u32::from(compared_to),
            ))?,
        })
    }
}

/// A time difference at the split level.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Split {
    /// The pace.
    pub pace: pace::SplitInRun,
    /// The absolute amount by which the split-level times differ.
    pub split_abs_delta: time::Time,
    /// The absolute amount by which the run-level times differ.
    pub run_abs_delta: time::Time,
}

impl Split {
    /// Constructs a [Split] delta from two [Delta]s: one representing the split individually,
    /// the other representing the cumulative delta across the run so far.
    #[must_use]
    pub fn new(split: Delta, run_so_far: Delta) -> Self {
        Self {
            pace: pace::SplitInRun::new(split.pace, run_so_far.pace),
            split_abs_delta: split.abs_delta,
            run_abs_delta: run_so_far.abs_delta,
        }
    }

    /// Reconstructs a run [Delta] from a split delta.
    #[must_use]
    pub fn run(&self) -> Delta {
        Delta {
            pace: self.pace.overall(),
            abs_delta: self.run_abs_delta,
        }
    }
}

/// A pair of a time and its delta against comparison.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Time {
    /// The delta.
    pub delta: Delta,
    /// The time to which `delta` applies.
    pub time: super::Time,
}

impl Time {
    /// Calculates the delta by comparing `time` to `compared_to`, and attaches it to `time`.
    ///
    /// # Errors
    ///
    /// Fails if we can't capture the delta as a [Time].
    pub fn of_comparison(time: time::Time, compared_to: time::Time) -> time::error::Result<Self> {
        let delta = Delta::of_comparison(time, compared_to)?;
        Ok(Time { delta, time })
    }

    /// Reconstitutes the comparison time by applying the delta to the attempt time.
    ///
    /// # Errors
    ///
    /// Fails if there is an underflow or overflow in the time production.
    ///
    /// ```
    /// use zombiesplit::model::timing;
    ///
    /// let t1 = timing::Time::seconds(123).unwrap();
    /// let t2 = timing::Time::seconds(456).unwrap();
    ///
    /// let d1 = timing::comparison::delta::Time::of_comparison(t1, t2).unwrap();
    /// let d2 = timing::comparison::delta::Time::of_comparison(t2, t1).unwrap();
    ///
    /// // Each of these should reconstitute the named original comparison time...
    /// let t2a = d1.comparison().unwrap();
    /// let t1a = d2.comparison().unwrap();
    ///
    /// assert_eq!(t1, t1a);
    /// assert_eq!(t2, t2a);
    /// ```
    pub fn comparison(&self) -> time::error::Result<super::Time> {
        let ms = u32::from(self.time);
        let delta = u32::from(self.delta.abs_delta);
        let mod_ms: u32 = match self.delta.pace {
            // TODO: eliminate PersonalBest here; if this is a run split, it's not necessarily ahead
            super::Pace::Ahead | super::Pace::PersonalBest => ms + delta,
            super::Pace::Behind => ms - delta,
            super::Pace::Inconclusive => ms,
        };
        super::super::Time::try_from(mod_ms)
    }

    /// Extracts the pace and time from this [Time] and puts them together.
    ///
    /// ```
    /// use zombiesplit::model::timing;
    ///
    /// let t1 = timing::Time::seconds(123).unwrap();
    /// let t2 = timing::Time::seconds(456).unwrap();
    ///
    /// let d = timing::comparison::delta::Time::of_comparison(t1, t2).unwrap();
    /// let p = d.paced_time();
    ///
    /// assert_eq!(timing::comparison::Pace::Ahead, p.pace);
    /// assert_eq!(t1, p.time);
    /// ```
    #[must_use]
    pub fn paced_time(&self) -> pace::PacedTime {
        pace::PacedTime {
            pace: self.delta.pace,
            time: self.time,
        }
    }
}
