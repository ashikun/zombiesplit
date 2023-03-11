//! Deltas between times.
//!
//! A delta is simply a time with a specific meaning.

use super::{super::time, pace};

/// A single-time time difference (current time - comparison time).
///
/// Usually, these will represent run-wide cumulative deltas, but they are also used to construct
/// [Split]s.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Delta(time::Time);

impl Delta {
    /// Gets the pace of this delta.
    #[must_use]
    pub fn pace(&self) -> pace::Pace {
        // TODO: personal best?
        if self.0.into_millis() <= 0 {
            pace::Pace::Ahead
        } else {
            pace::Pace::Behind
        }
    }

    /// Calculates the delta by comparing `time` to `compared_to`, if it exists.
    #[must_use]
    pub fn of_comparison(time: time::Time, compared_to: time::Time) -> Self {
        Self(time - compared_to)
    }
}

/// A time difference at the split level.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Split {
    /// The split delta.
    pub split: Delta,
    /// The cumulative run delta.
    pub run: Delta,
}

impl Split {
    /// Constructs a [Split] delta from two [Delta]s: one representing the split individually,
    /// the other representing the cumulative delta across the run so far.
    #[must_use]
    pub fn new(split: Delta, run: Delta) -> Self {
        Self { split, run }
    }

    /// Gets the split-in-run pace from this delta.
    #[must_use]
    pub fn pace(&self) -> pace::SplitInRun {
        pace::SplitInRun::new(self.split.pace(), self.run.pace())
    }
}

/// A pair of a time and its delta against comparison.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Time {
    /// The delta.
    pub delta: Delta,
    /// The time to which `delta` applies.
    pub time: time::Time,
}

impl Time {
    /// Calculates the delta by comparing `time` to `compared_to`, and attaches it to `time`.
    #[must_use]
    pub fn of_comparison(time: time::Time, compared_to: time::Time) -> Self {
        let delta = Delta::of_comparison(time, compared_to);
        Time { delta, time }
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
    /// let t1 = timing::time::Time::from_millis(123);
    /// let t2 = timing::time::Time::from_millis(456);
    ///
    /// let d1 = timing::comparison::delta::Time::of_comparison(t1, t2);
    /// let d2 = timing::comparison::delta::Time::of_comparison(t2, t1);
    ///
    /// // Each of these should reconstitute the named original comparison time...
    /// let t2a = d1.comparison();
    /// let t1a = d2.comparison();
    ///
    /// assert_eq!(t1, t1a);
    /// assert_eq!(t2, t2a);
    /// ```
    pub fn comparison(&self) -> time::Time {
        // comparison = time - (time - comparison)
        self.time - self.delta.0
    }

    /// Extracts the pace and time from this [Time] and puts them together.
    ///
    /// ```
    /// use zombiesplit::model::timing;
    ///
    /// let t1 = timing::time::Time::from_millis(123);
    /// let t2 = timing::time::Time::from_millis(456);
    ///
    /// let d = timing::comparison::delta::Time::of_comparison(t1, t2);
    /// let p = d.paced_time();
    ///
    /// assert_eq!(timing::comparison::Pace::Ahead, p.pace);
    /// assert_eq!(t1, p.time);
    /// ```
    #[must_use]
    pub fn paced_time(&self) -> pace::PacedTime {
        pace::PacedTime {
            pace: self.delta.pace(),
            time: self.time,
        }
    }
}
