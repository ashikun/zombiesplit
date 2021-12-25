//! zombiesplit's notion of times.
use std::ops::Index;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
    iter::Sum,
    str::FromStr,
};

use serde_with::{DeserializeFromStr, SerializeDisplay};

pub use error::Error;
pub use field::{Field, Hour, Minute, Msec, Second};
pub use format::Format;

pub mod carry;
pub mod error;
pub mod field;
pub mod format;
pub mod position;

/// A hh:mm:ss:ms timing.
#[derive(
    Copy,
    Clone,
    Default,
    SerializeDisplay,
    DeserializeFromStr,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Time {
    /// Number of hours.
    pub hours: Hour,
    /// Number of minutes.
    pub mins: Minute,
    /// Number of seconds.
    pub secs: Second,
    /// Number of milliseconds.
    pub millis: Msec,
}

impl Time {
    /// Tries to construct a [Time] from a given number of seconds.
    ///
    /// This is generally useful for testing.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::time::Time;
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

    /// Tries to set the field at `position` from string `str`.
    ///
    /// # Errors
    ///
    /// Fails if the string does not parse properly for the particular position.
    pub fn set_field_str(&mut self, position: position::Index, str: &str) -> error::Result<()> {
        // TODO(@MattWindsor91): do this more elegantly.
        match position {
            position::Index::Hours => self.hours = str.parse()?,
            position::Index::Minutes => self.mins = str.parse()?,
            position::Index::Seconds => self.secs = str.parse()?,
            position::Index::Milliseconds => self.millis = str.parse()?,
        };
        Ok(())
    }

    /// Gets whether this time is zero.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::time::Time;
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
    /// use zombiesplit::model::time::Time;
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
        let millis = Msec::new_with_carry(stamp);
        let secs = Second::new_with_carry(millis.carry);
        let mins = Minute::new_with_carry(secs.carry);
        let hours = Hour::try_from(mins.carry)?;
        Ok(Self {
            hours,
            mins: mins.value,
            secs: secs.value,
            millis: millis.value,
        })
    }
}

impl From<Time> for u32 {
    /// Converts a time to a 32-bit timestamp.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::model::time::Time;
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
        let (hours, s) = Hour::parse_delimited(s)?;
        let (mins, s) = Minute::parse_delimited(s)?;
        let (secs, s) = Second::parse_delimited(s)?;
        let millis = s.parse()?;
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
impl Index<position::Index> for Time {
    type Output = dyn field::Any;

    fn index(&self, index: position::Index) -> &Self::Output {
        match index {
            position::Index::Hours => &self.hours,
            position::Index::Minutes => &self.mins,
            position::Index::Seconds => &self.secs,
            position::Index::Milliseconds => &self.millis,
        }
    }
}

mod tests {
    /// Tests that adding and subtracting a time appears to be the identity.
    #[test]
    fn time_add_sub() {
        let t1: super::Time = "1h5m10s".parse().expect("should be valid");
        let t2: super::Time = "6m4s100".parse().expect("should be valid");
        assert_eq!(t1, (t1 + t2) - t2);
    }

    /// Tests that subtracting a large time from a short time results in zero.
    #[test]
    fn time_sub_sat() {
        let t1: super::Time = "1h5m10s".parse().expect("should be valid");
        let t2: super::Time = "6m4s100".parse().expect("should be valid");
        assert_eq!(super::Time::default(), t2 - t1);
    }

    #[test]
    fn time_from_str_empty() {
        let t: super::Time = "".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 0);
        assert_eq!(u16::from(t.millis), 0);
    }

    #[test]
    fn time_from_str_msec_only() {
        // This case may be removed later on, it's a bit weird.
        let t: super::Time = "123".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 0);
        assert_eq!(u16::from(t.millis), 123);
    }

    #[test]
    fn time_from_str_msec_short() {
        let t: super::Time = "02".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 0);
        assert_eq!(u16::from(t.millis), 20);
    }

    #[test]
    fn time_from_str_secs_only() {
        let t: super::Time = "10s".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 10);
        assert_eq!(u16::from(t.millis), 0);
    }

    #[test]
    fn time_from_str_secs_msec() {
        let t: super::Time = "10s50".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 0);
        assert_eq!(u16::from(t.mins), 0);
        assert_eq!(u16::from(t.secs), 10);
        assert_eq!(u16::from(t.millis), 500);
    }

    #[test]
    fn time_from_str_all() {
        let t: super::Time = "1h2m3s456".parse().expect("should be valid");
        assert_eq!(u16::from(t.hours), 1);
        assert_eq!(u16::from(t.mins), 2);
        assert_eq!(u16::from(t.secs), 3);
        assert_eq!(u16::from(t.millis), 456);
    }

    /// Tests that indexing seems to work properly.
    #[test]
    fn index() {
        let t: super::Time = "1h2m3s456".parse().expect("should be valid");
        assert_eq!("01", t[super::position::Index::Hours].to_string());
        assert_eq!("02", t[super::position::Index::Minutes].to_string());
        assert_eq!("03", t[super::position::Index::Seconds].to_string());
        assert_eq!("456", t[super::position::Index::Milliseconds].to_string());
    }
}
