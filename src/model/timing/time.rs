//! zombiesplit's notion of times.
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display},
    iter::Sum,
    ops::{Index, IndexMut},
    str::FromStr,
};

use serde_with::{DeserializeFromStr, SerializeDisplay};

pub use error::Error;
pub use field::Field;
pub use format::Format;
pub use position::Position;

pub mod error;
pub mod field;
pub mod format;
pub mod position;

/// A hh:mm:ss:ms timing.
#[derive(Copy, Clone, SerializeDisplay, DeserializeFromStr, Debug, PartialEq, Eq, Hash)]
pub struct Time {
    /// Number of hours.
    pub hours: Field,
    /// Number of minutes.
    pub mins: Field,
    /// Number of seconds.
    pub secs: Field,
    /// Number of milliseconds.
    pub millis: Field,
}

/// This cannot be autoderived as fields don't have a default (what would the default position be?);
/// however, an invariant of times is that the fields' positions line up with their parts of the
/// struct, and so we can fill in accordingly.
impl Default for Time {
    fn default() -> Self {
        Time {
            hours: Field::zero(Position::Hours),
            mins: Field::zero(Position::Minutes),
            secs: Field::zero(Position::Seconds),
            millis: Field::zero(Position::Milliseconds),
        }
    }
}

/// This cannot be autoderived because fields carry their position.
impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        u32::from(*self).cmp(&u32::from(*other))
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Time {
    /// Constructs a time from its hour, minute, second, and millisecond positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::time::Time;
    ///
    /// let t1 = Time::new(1, 23, 45, 678).expect("shouldn't overflow");
    /// assert_eq!(1, u16::from(t1.hours));
    /// assert_eq!(23, u16::from(t1.mins));
    /// assert_eq!(45, u16::from(t1.secs));
    /// assert_eq!(678, u16::from(t1.millis));
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if the amount for any field is too high to store
    pub fn new(hours: u32, mins: u32, secs: u32, millis: u32) -> error::Result<Self> {
        Ok(Self {
            hours: Field::new(Position::Hours, hours)?,
            mins: Field::new(Position::Minutes, mins)?,
            secs: Field::new(Position::Seconds, secs)?,
            millis: Field::new(Position::Milliseconds, millis)?,
        })
    }

    /// Tries to construct a [Time] from a given number of seconds.
    ///
    /// This is generally useful for testing.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::time::Time;
    ///
    /// let t1 = Time::seconds(0).expect("shouldn't overflow");
    /// assert!(t1.is_zero());
    ///
    /// let t2 = Time::seconds(42).expect("shouldn't overflow");
    /// assert_eq!(42, u16::from(t2.secs));
    ///
    /// let t3 = Time::seconds(182).expect("shouldn't overflow");
    /// assert_eq!(3, u16::from(t3.mins));
    /// assert_eq!(2, u16::from(t3.secs));
    /// ```
    ///
    /// # Errors
    ///
    /// Errors if the number of seconds is too high to store in this time.
    pub fn seconds(amount: u32) -> error::Result<Self> {
        // TODO(@MattWindsor91): error if overflowing multiplication
        Self::try_from(amount * 1000)
    }

    /// Gets whether this time is zero.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::timing::time::Time;
    /// use std::convert::TryFrom;
    /// assert!(Time::try_from(0).expect("shouldn't overflow").is_zero());
    /// assert!(!Time::try_from(1).expect("shouldn't overflow").is_zero());
    /// ```
    #[must_use]
    pub fn is_zero(self) -> bool {
        u32::from(self) == 0
    }
}

impl TryFrom<u32> for Time {
    type Error = error::Error;

    /// Tries to convert a 32-bit timestamp to a time.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::timing::time::Time;
    /// use std::convert::TryFrom;
    /// let time = Time::try_from(
    ///     789 + (56 * 1000) + (34 * 1000 * 60) + (12 * 1000 * 60 * 60)
    /// ).expect("should not overflow");
    /// assert_eq!(u16::from(time.hours), 12);
    /// assert_eq!(u16::from(time.mins), 34);
    /// assert_eq!(u16::from(time.secs), 56);
    /// assert_eq!(u16::from(time.millis), 789);
    /// ```
    fn try_from(stamp: u32) -> Result<Self, Self::Error> {
        let millis = field::Carry::new(Position::Milliseconds, stamp);
        let secs = field::Carry::new(Position::Seconds, millis.carry);
        let mins = field::Carry::new(Position::Minutes, secs.carry);
        let hours = Field::new(Position::Hours, mins.carry)?;
        Ok(Self {
            hours,
            mins: mins.field,
            secs: secs.field,
            millis: millis.field,
        })
    }
}

impl From<Time> for u32 {
    /// Converts a time to a 32-bit timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::timing::time::Time;
    /// use std::convert::TryFrom;
    /// let msec = 1234567;
    /// let time = Time::try_from(msec).expect("should not overflow");
    /// assert_eq!(u32::from(time), msec);
    /// ```
    fn from(time: Time) -> u32 {
        time.millis.as_msecs() + time.secs.as_msecs() + time.mins.as_msecs() + time.hours.as_msecs()
    }
}

impl Sum for Time {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let raw: u32 = iter.map(u32::from).sum();
        Self::try_from(raw).unwrap_or_default()
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO(@MattWindsor91): make this more efficient?
        let raw: u32 = u32::from(self) + u32::from(rhs);
        Self::try_from(raw).unwrap_or_default()
    }
}

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        // TODO(@MattWindsor91): make this more efficient?
        let raw: u32 = u32::from(self).saturating_sub(u32::from(rhs));
        Self::try_from(raw).unwrap_or_default()
    }
}

impl std::ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        // TODO(@MattWindsor91): make this more efficient?
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        // TODO(@MattWindsor91): make this more efficient?
        *self = *self - rhs;
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.hours.fmt_value_delimited(f)?;
        self.mins.fmt_value_delimited(f)?;
        self.secs.fmt_value_delimited(f)?;
        self.millis.fmt_value_delimited(f)?;
        Ok(())
    }
}

impl FromStr for Time {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hours, s) = Field::parse_delimited(Position::Hours, s)?;
        let (mins, s) = Field::parse_delimited(Position::Minutes, s)?;
        let (secs, s) = Field::parse_delimited(Position::Seconds, s)?;
        let millis = Field::parse(Position::Milliseconds, s)?;
        Ok(Self {
            hours,
            mins,
            secs,
            millis,
        })
    }
}

impl rusqlite::types::FromSql for Time {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        u32::column_result(value).and_then(|x| {
            Self::try_from(x).map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
        })
    }
}

impl rusqlite::ToSql for Time {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(i64::from(u32::from(*self))),
        ))
    }
}

/// We can index into a time by position index, returning a field.
impl Index<Position> for Time {
    type Output = Field;

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

    /// Tests that adding and subtracting a time appears to be the identity.
    #[test]
    fn time_add_sub() {
        let t1: Time = "1h5m10s".parse().expect("should be valid");
        let t2: Time = "6m4s100".parse().expect("should be valid");
        assert_eq!(t1, (t1 + t2) - t2);
    }

    /// Tests that subtracting a large time from a short time results in zero.
    #[test]
    fn time_sub_sat() {
        let t1: Time = "1h5m10s".parse().expect("should be valid");
        let t2: Time = "6m4s100".parse().expect("should be valid");
        assert_eq!(Time::default(), t2 - t1);
    }

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
