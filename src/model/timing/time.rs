//! zombiesplit's notion of times.

pub use error::Error;
pub use format::Format;
pub use position::Position;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub mod error;
pub mod field;
pub mod format;
pub mod human;
pub mod position;

/// A time.
///
/// Times (more correctly, durations) are represented internally as a number of milliseconds.
/// The exact representation is subject to change, but should be enough to accommodate all but the
/// most pathological speedrun cumulative times (eg, it can stretch to several days before
/// overflow).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash)]
pub struct Time(i32);

impl Time {
    /// Creates a time from a signed millisecond representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::time::Time;
    ///
    /// // round-trip
    /// assert_eq!(42, Time::from_millis(42).into_millis())
    /// ```
    #[must_use]
    pub const fn from_millis(amount: i32) -> Self {
        Self(amount)
    }

    /// Converts this time to a signed millisecond representation.
    #[must_use]
    pub const fn into_millis(self) -> i32 {
        self.0
    }
}

impl Sum for Time {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from_millis(iter.map(Self::into_millis).sum())
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl rusqlite::types::FromSql for Time {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        i32::column_result(value).map(Self)
    }
}

impl rusqlite::ToSql for Time {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(i64::from(self.0)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that adding and subtracting a time appears to be the identity.
    #[test]
    fn time_add_sub() {
        let t1: Time = Time::from_millis(15100);
        let t2: Time = Time::from_millis(6410);
        assert_eq!(t1, (t1 + t2) - t2);
    }

    /// Tests that subtracting a large time from a short time does not saturate.
    #[test]
    fn time_sub_no_sat() {
        let t1: Time = Time::from_millis(15100);
        let t2: Time = Time::from_millis(6410);
        assert_eq!(t2, (t2 - t1) + t2);
    }
}
