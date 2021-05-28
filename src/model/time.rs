//! zombiesplit's notion of times.
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    convert::TryFrom,
    fmt::{self, Display},
    iter::Sum,
    str::FromStr,
};

pub mod carry;
pub mod error;
pub mod field;
pub mod position;

pub use error::Error;
pub use field::Field;
pub use position::{Hour, Minute, Msec, Second};

/// A hh:mm:ss:ms timing.
#[derive(Copy, Clone, SerializeDisplay, DeserializeFromStr, Debug)]
pub struct Time {
    /// Number of hours.
    pub hours: Field<Hour>,
    /// Number of minutes.
    pub mins: Field<Minute>,
    /// Number of seconds.
    pub secs: Field<Second>,
    /// Number of milliseconds.
    pub millis: Field<Msec>,
}

impl Time {
    /// Tries to set the field at `position` from string `str`.
    ///
    /// # Errors
    ///
    /// Fails if the string does not parse properly for the particular position.
    pub fn set_field_str(&mut self, position: position::Name, str: &str) -> error::Result<()> {
        // TODO(@MattWindsor91): do this more elegantly.
        match position {
            position::Name::Hours => self.hours = str.parse()?,
            position::Name::Minutes => self.mins = str.parse()?,
            position::Name::Seconds => self.secs = str.parse()?,
            position::Name::Milliseconds => self.millis = str.parse()?,
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
        let millis = Field::<Msec>::new_with_carry(stamp);
        let secs = Field::new_with_carry(millis.carry);
        let mins = Field::new_with_carry(secs.carry);
        let hours = Field::try_from(mins.carry)?;
        Ok(Self {
            hours,
            mins: mins.value,
            secs: secs.value,
            millis: millis.value,
        })
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            millis: Field::default(),
            secs: Field::default(),
            mins: Field::default(),
            hours: Field::default(),
        }
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
        let raw: u32 = u32::from(self) + u32::from(rhs);
        Self::try_from(raw).unwrap_or_default()
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
        let (hours, s) = Field::<Hour>::parse_delimited(s)?;
        let (mins, s) = Field::<Minute>::parse_delimited(s)?;
        let (secs, s) = Field::<Second>::parse_delimited(s)?;
        let millis = s.parse()?;
        Ok(Self {
            hours,
            mins,
            secs,
            millis,
        })
    }
}

mod tests {
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
}
