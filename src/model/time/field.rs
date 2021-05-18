//! Time fields and associated functions.
use super::{
    error::{Error, Result},
    position::Position,
};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    marker::PhantomData,
    str::FromStr,
};

/// A field in a time struct.
pub struct Field<P> {
    /// The value.
    val: u16,
    /// The phantom type.
    phantom: PhantomData<*const P>,
}

// The phantom type makes derivations difficult.

impl<P> Clone for Field<P> {
    fn clone(&self) -> Self {
        Self::new(self.val)
    }
}

impl<P> Copy for Field<P> {}

impl<P> std::fmt::Debug for Field<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.val.fmt(f)
    }
}

impl<P> Default for Field<P> {
    fn default() -> Self {
        Self::new(0)
    }
}

/// We can extract the value of the field, regardless of position.
impl<P> From<Field<P>> for u16 {
    fn from(field: Field<P>) -> u16 {
        field.val
    }
}

impl<P: Position> TryFrom<u32> for Field<P> {
    type Error = Error;

    /// Tries to fit `val` into this field.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use zombiesplit::model::time::{Field, Second};
    ///
    /// let f1 = Field::<Second>::try_from(4);
    /// assert!(f1.is_ok(), "shouldn't have overflowed");
    /// let f2 = Field::<Second>::try_from(64);
    /// assert!(!f2.is_ok(), "should have overflowed");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::FieldTooBig` if `val` is too large for the field.
    fn try_from(val: u32) -> Result<Self> {
        let (field, carry) = Self::new_with_carry(val);
        if carry == 0 {
            Ok(field)
        } else {
            Err(Error::FieldTooBig {
                pos: P::name(),
                val,
            })
        }
    }
}

impl<P: Position> FromStr for Field<P> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        let val: u32 = P::preprocess_string(s)
            .parse()
            .map_err(|err| Error::FieldParse {
                pos: P::name(),
                err,
            })?;
        Self::try_from(val)
    }
}

impl<P> Field<P> {
    /// Creates a new field with the given value.
    ///
    /// This is not widely exposed to avoid the invariant (val is between 0 and
    /// position maximum) being broken.
    #[must_use]
    fn new(val: u16) -> Self {
        Self {
            val,
            phantom: PhantomData::default(),
        }
    }
}

impl<P: Position> Field<P> {
    /// Formats the value `v` with a delimiter, if nonzero.
    pub(super) fn fmt_value_delimited(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::fmt_value_delimited(self.val, f)
    }

    /// Tries to find this field's position delimiter in `s`, and parses the
    /// delimited number if one exists; otherwise, returns an empty field.
    pub(super) fn parse_delimited(s: &str) -> Result<(Self, &str)> {
        let (to_parse, to_return) = P::split_delimiter(s);
        Ok((to_parse.parse()?, to_return))
    }

    /// Fits as much of `val` as possible into a field, and returns the field
    /// and any carry.
    ///
    /// ```
    /// use zombiesplit::model::time::{Field, Second};
    /// let (field, carry) = Field::<Second>::new_with_carry(64);
    /// assert_eq!(u16::from(field), 4, "should have taken 4 seconds");
    /// assert_eq!(carry, 1, "should have carried over 1 minute")
    /// ```
    #[must_use]
    pub fn new_with_carry(val: u32) -> (Self, u32) {
        let (carry, val) = div_rem(val, P::cap());
        (Self::new(val.try_into().unwrap_or_default()), carry)
    }
    /// Returns this field's value as milliseconds.
    ///
    /// ```
    /// use zombiesplit::model::time::{Field, Second};
    /// use std::convert::TryFrom;
    /// let msec = Field::<Second>::try_from(20).unwrap().as_msecs();
    /// assert_eq!(msec, 20_000, "20 secs = 20,000 msecs");
    /// ```
    #[must_use]
    pub fn as_msecs(self) -> u32 {
        u32::from(self.val) * P::ms_offset()
    }
}

fn div_rem(x: u32, divisor: u32) -> (u32, u32) {
    let rem = x % divisor;
    ((x - rem) / divisor, rem)
}
