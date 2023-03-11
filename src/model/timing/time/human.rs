//! Human-centred presentations of times.

use super::{
    error::{Error, Result},
    Position,
};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt,
    ops::{Index, IndexMut},
    str::FromStr,
};

/// A time split into hours, minutes, seconds, and milliseconds for human consumption.
///
/// Internally, zombiesplit represents times in milliseconds where possible, but this is cumbersome
/// for both displaying to users and storing on disk.  This representation provides a way to
/// access the individual fields of a timestamp easily.
///
/// Note that not all human-times are valid times: some are too large to represent, and negative
/// zero is subnormal.
#[derive(
    Copy,
    Clone,
    Default,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Time {
    /// Whether the time is negative.
    pub is_negative: bool,
    /// Number of hours (0-596).
    pub hours: u16,
    /// Number of minutes (0-59).
    pub mins: u16,
    /// Number of seconds (0-59).
    pub secs: u16,
    /// Number of milliseconds (0-999).
    pub millis: u16,
}

impl Time {
    /// Constructs a non-negative time from its hour, minute, second, and millisecond positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::time::human;
    ///
    /// let t1 = human::Time::new(1, 23, 45, 678);
    /// assert!(!t1.is_negative);
    /// assert_eq!(1, t1.hours);
    /// assert_eq!(23, t1.mins);
    /// assert_eq!(45, t1.secs);
    /// assert_eq!(678, t1.millis);
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if the amount for any field is too high to store.
    pub fn new(hours: u16, mins: u16, secs: u16, millis: u16) -> Self {
        Self {
            is_negative: false,
            hours,
            mins,
            secs,
            millis,
        }
    }

    /// Converts a time given as a 32-bit millisecond timestamp to a human time.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::timing::time::human;
    /// use std::convert::From;
    /// let time = human::Time::from_millis(
    ///     789 + (56 * 1000) + (34 * 1000 * 60) + (12 * 1000 * 60 * 60)
    /// );
    /// assert_eq!(u16::from(time.hours), 12);
    /// assert_eq!(u16::from(time.mins), 34);
    /// assert_eq!(u16::from(time.secs), 56);
    /// assert_eq!(u16::from(time.millis), 789);
    /// ```
    pub fn from_millis(millis: i32) -> Self {
        let mut result = Self::default();
        // What we put here is not important, as it'll get overwritten immediately.
        // The exception is the overflow, which needs to start off as the whole timestamp.
        let mut fit = super::position::Fit {
            overflow: millis.unsigned_abs(),
            input: 0,
            result: super::position::Value {
                value: 0,
                field: Position::Milliseconds,
            },
        };

        // Reverse iterator gives us millis, secs, mins, hours.
        for p in Position::ALL.iter().rev() {
            fit = p.fit(fit.overflow);
            assert_eq!(fit.result.field, *p, "fit should return the same field");
            result[*p] = fit.result.value;
        }

        result.is_negative = millis.is_negative();
        result
    }

    /// Converts a [Time] into a 32-bit millisecond timestamp.
    ///
    /// # Errors
    ///
    /// Fails if the time doesn't fit into a signed 32-bit integer.
    pub fn try_into_millis(self) -> Result<i32> {
        Position::ALL.iter().map(|f| self.field_ms(*f)).sum()
    }

    /// Tries to construct a [Time] from a given number of seconds.
    ///
    /// This is generally useful for testing.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::time::human;
    ///
    /// let t1 = human::Time::seconds(0).expect("shouldn't overflow");
    /// assert!(t1.is_zero());
    ///
    /// let t2 = human::Time::seconds(42).expect("shouldn't overflow");
    /// assert_eq!(42, u16::from(t2.secs));
    ///
    /// let t3 = human::Time::seconds(182).expect("shouldn't overflow");
    /// assert_eq!(3, u16::from(t3.mins));
    /// assert_eq!(2, u16::from(t3.secs));
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if the number of seconds is too high to store in this time.
    pub fn seconds(amount: i32) -> Result<Self> {
        Ok(Self::from_millis(
            amount.checked_mul(1000).ok_or(Error::SecOverflow(amount))?,
        ))
    }

    /// Gets whether this time is zero.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::timing::time::human;
    /// use std::convert::TryFrom;
    /// assert!(human::Time::try_from(0).unwrap().is_zero());
    /// assert!(!human::Time::try_from(1).unwrap().is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.hours == 0 && self.mins == 0 && self.secs == 0 && self.millis == 0
    }

    fn field_ms(&self, field: Position) -> Result<i32> {
        let offset = i32::from(field.ms_offset());
        let base = i32::from(self[field]);
        offset.checked_mul(base).ok_or(Error::MsecOverflow(*self))
    }
}

//
// Conversion to and from internal times
//

/// Conversion from internal times to human times.
impl From<super::Time> for Time {
    fn from(time: super::Time) -> Self {
        Self::from_millis(time.into_millis())
    }
}

/// Partial conversion from human times to internal times.
impl TryFrom<Time> for super::Time {
    type Error = super::Error;

    /// Converts a time to an internal timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::timing::time::human;
    /// use std::convert::TryFrom;
    /// let msec = 1234567;
    /// let time = human::Time::try_from(msec).expect("should not overflow");
    /// assert_eq!(u32::from(time), msec);
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the time is too large to fit into a timestamps.
    fn try_from(time: Time) -> Result<super::Time> {
        let millis: Result<i32> = time.try_into_millis();
        millis.map(super::Time::from_millis)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Position::ALL
            .iter()
            .try_for_each(|p| p.fmt_value(f, self[*p]))
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut result = Self::default();
        let mut rest = s;
        for p in Position::ALL {
            let (val, r) = p.split_and_parse(s)?;
            result[*p] = val;
            rest = r;
        }
        Ok(result)
    }
}

/// We can index into a time by position index, returning a field.
impl Index<Position> for Time {
    type Output = u16;

    fn index(&self, index: Position) -> &Self::Output {
        match index {
            Position::Hours => &self.hours,
            Position::Minutes => &self.mins,
            Position::Seconds => &self.secs,
            Position::Milliseconds => &self.millis,
        }
    }
}

impl IndexMut<Position> for Time {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        match index {
            Position::Hours => &mut self.hours,
            Position::Minutes => &mut self.mins,
            Position::Seconds => &mut self.secs,
            Position::Milliseconds => &mut self.millis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_from_str_empty() {
        let t: Time = "".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 0);
        assert_eq!(u16::from(t.millis), 0);
    }

    #[test]
    fn time_from_str_msec_only() {
        // This case may be removed later on, it's a bit weird.
        let t: Time = "123".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 0);
        assert_eq!(u16::from(t.millis), 123);
    }

    #[test]
    fn time_from_str_msec_short() {
        let t: Time = "02".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 0);
        assert_eq!(u16::from(t.millis), 20);
    }

    #[test]
    fn time_from_str_secs_only() {
        let t: Time = "10s".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 10);
        assert_eq!(u16::from(t.millis), 0);
    }

    #[test]
    fn time_from_str_secs_msec() {
        let t: Time = "10s50".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 10);
        assert_eq!(u16::from(t.millis), 500);
    }

    #[test]
    fn time_from_str_all() {
        let t: Time = "1h2m3s456".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 1);
        assert_eq!(u16::from(t.mins), 2);
        assert_eq!(u16::from(t.secs), 3);
        assert_eq!(u16::from(t.millis), 456);
    }

    /// Tests that indexing seems to work properly.
    #[test]
    fn index() {
        let t: Time = "1h2m3s456".parse().expect("should be valid");
        assert_eq!("01", t[Position::Hours].to_string());
        assert_eq!("02", t[Position::Minutes].to_string());
        assert_eq!("03", t[Position::Seconds].to_string());
        assert_eq!("456", t[Position::Milliseconds].to_string());
    }
}
