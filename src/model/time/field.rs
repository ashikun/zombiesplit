//! Time fields and associated functions.
use super::{
    carry::{self, Carry},
    error::{Error, Result},
    position::{self, Marker},
};
use std::hash::{Hash, Hasher};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    marker::PhantomData,
    str::FromStr,
};

/// A field in a time struct, containing a phantom position marker type.
///
/// To get the value of a field, convert it to a `u16` using [From]/[Into].
pub struct Field<P> {
    /// The value.
    val: u16,
    /// The phantom type.
    phantom: PhantomData<*const P>,
}

/// Object-safe, position-erased trait for fields.
///
/// This is used to make it possible to be generic over the particular position marker in use.
pub trait Any: fmt::Display {
    // Move things into this trait as soon as we need them.

    /// Parses an input string into this field.
    ///
    /// It isn't possible to make [Any] a subtrait of `std::str::FromStr` for object safety and
    /// sizedness reasons, so this is the next best thing.
    ///
    /// # Errors
    ///
    /// Fails if the parse operation fails for any reason.
    fn parse_from(&mut self, input: &str) -> Result<()>;
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

impl<P> Hash for Field<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

/// We can extract the value of the field, regardless of position.
impl<P> From<Field<P>> for u16 {
    fn from(field: Field<P>) -> u16 {
        field.val
    }
}

impl<P> PartialEq for Field<P> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<P> Eq for Field<P> {}

impl<P> PartialOrd for Field<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<P> Ord for Field<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl<P: Marker> TryFrom<u32> for Field<P> {
    type Error = Error;

    /// Tries to fit `val` into this field.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use zombiesplit::model::time::Second;
    ///
    /// let f1 = Second::try_from(4);
    /// assert!(f1.is_ok(), "shouldn't have overflowed");
    /// let f2 = Second::try_from(64);
    /// assert!(!f2.is_ok(), "should have overflowed");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::FieldTooBig` if `val` is too large for the field.
    fn try_from(val: u32) -> Result<Self> {
        Self::new_with_carry(val).try_into()
    }
}

impl<P: Marker> fmt::Display for Field<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::fmt_value(self.val, f)
    }
}

impl<P: Marker> FromStr for Field<P> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Ok(Self::default());
        }

        let val: u32 = P::preprocess_string(s)
            .parse()
            .map_err(|err| Error::FieldParse {
                pos: P::position(),
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

impl<P: Marker> Field<P> {
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
    /// use zombiesplit::model::time::Second;
    /// let result = Second::new_with_carry(64);
    /// assert_eq!(u16::from(result.value), 4, "should have taken 4 seconds");
    /// assert_eq!(result.carry, 1, "should have carried over 1 minute")
    /// ```
    #[must_use]
    pub fn new_with_carry(val: u32) -> Carry<Self> {
        Carry::from_division(val, P::cap()).map(|x| Self::new(x.try_into().unwrap_or_default()))
    }
    /// Returns this field's value as milliseconds.
    ///
    /// ```
    /// use zombiesplit::model::time::Second;
    /// use std::convert::TryFrom;
    /// let msec = Second::try_from(20).unwrap().as_msecs();
    /// assert_eq!(msec, 20_000, "20 secs = 20,000 msecs");
    /// ```
    #[must_use]
    pub fn as_msecs(self) -> u32 {
        u32::from(self.val) * P::ms_offset()
    }
}

impl<P: Marker> TryFrom<carry::Carry<Field<P>>> for Field<P> {
    type Error = Error;

    fn try_from(c: carry::Carry<Field<P>>) -> Result<Field<P>> {
        if c.carry == 0 {
            Ok(c.value)
        } else {
            Err(Error::FieldTooBig {
                pos: P::position(),
                val: c.original,
            })
        }
    }
}

/// Erasing the position field.
impl<P: Marker> Any for Field<P> {
    fn parse_from(&mut self, input: &str) -> Result<()> {
        // TODO(@MattWindsor91): do this more efficiently
        *self = input.parse()?;
        Ok(())
    }
}

/// Shorthand for an hour field.
pub type Hour = Field<position::Hour>;
/// Shorthand for a minute field.
pub type Minute = Field<position::Minute>;
/// Shorthand for a second field.
pub type Second = Field<position::Second>;
/// Shorthand for a millisecond field.
pub type Msec = Field<position::Msec>;

#[cfg(test)]
mod tests {
    /// Tests that the unusual parsing behaviour of milliseconds works properly.
    mod msec {
        /// Tests a millisecond parse.
        fn test_parse(from: &'static str, want: u16) {
            let t: super::super::Msec = from.parse().expect("should be valid");
            assert_eq!(u16::from(t), want);
        }

        /// Tests a millisecond display.
        fn test_display(from: u16, want: &'static str) {
            let t: super::super::Msec = super::super::Msec::new(from);
            assert_eq!(format!("{}", t), want);
        }

        /// Tests that the empty string is parsed into the right msec.
        #[test]
        fn from_str_empty() {
            test_parse("", 0);
        }

        /// Tests that a short unpadded string is parsed into the right msec.
        #[test]
        fn from_str_short() {
            test_parse("2", 200);
        }

        /// Tests that a full unpadded string is parsed into the right msec.
        #[test]
        fn from_str_full() {
            test_parse("246", 246);
        }

        /// Tests that a short half-padded string is parsed into the right msec.
        #[test]
        fn from_str_short_leading_zero() {
            test_parse("02", 20);
        }

        /// Tests that a full half-padded string is parsed into the right msec.
        #[test]
        fn from_str_full_leading_zero() {
            test_parse("024", 24);
        }

        /// Tests that a short fully-padded string is parsed into the right msec.
        #[test]
        fn from_str_short_leading_zeroes() {
            test_parse("002", 2);
        }

        /// Tests that a string full of zeroes is parsed into the right msec.
        #[test]
        fn from_str_all_zeroes() {
            test_parse("000", 0);
        }

        /* These tests mainly guard against the resurfacing of a bug
         * (reported and fixed by @Taneb) where the right padding used in
         * parsing was accidentally replicated in display, causing eg. 8ms
         * to be displayed as 800ms.
         */

        /// Tests that an empty millisecond field is padded into three zeroes.
        #[test]
        fn display_empty() {
            test_display(0, "000");
        }

        /// Tests that an one-digit millisecond field is padded correctly.
        #[test]
        fn display_one_digit() {
            test_display(1, "001");
        }

        /// Tests that a two-digit millisecond field is padded correctly.
        #[test]
        fn display_two_digits() {
            test_display(23, "023");
        }

        /// Tests that a three-digit millisecond field is padded correctly.
        #[test]
        fn display_three_digits() {
            test_display(456, "456");
        }

        /// Tests that an three-digit millisecond field with one trailing zero
        /// is padded correctly.
        #[test]
        fn display_three_digits_one_zero() {
            test_display(780, "780");
        }

        /// Tests that an three-digit millisecond field with two trailing zeroes
        /// is padded correctly.
        #[test]
        fn display_three_digits_two_zeroes() {
            test_display(900, "900");
        }

        // These next tests check what happens when we override the width on displaying
        // milliseconds.

        /// Tests that truncating a millisecond field to two digits drops digits from the right.
        #[test]
        fn display_three_digits_as_two() {
            let t: super::super::Msec = super::super::Msec::new(123);
            assert_eq!(format!("{:2}", t), "12");
        }

        /// Tests that stretching a millisecond field to four digits zero-pads on the left.
        #[test]
        fn display_three_digits_as_four() {
            let t: super::super::Msec = super::super::Msec::new(123);
            assert_eq!(format!("{:4}", t), "0123");
        }
    }
}
