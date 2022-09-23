//! Time fields and associated functions.
use super::{
    error::{Error, Result},
    Position,
};
use std::fmt::Write;
use std::hash::Hash;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

/// A field in a time struct.
///
/// Fields are only ordered if they have the same position.
///
/// To get the value of a field, convert it to a `u16` using [From]/[Into].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Field {
    // We don't expose these fields because doing so would allow invariant breakage:
    // 1. the maximum value of `val` is `position.cap() - 1`;
    // 2. `Time` relies on the fields' positions lining up with their parts of the struct.
    /// The value, in terms of the units of `position`.
    val: u16,
    /// The position this field represents.
    position: Position,
}

/// We can extract the value of the field, regardless of position.
impl From<Field> for u16 {
    fn from(field: Field) -> u16 {
        field.val
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.position == other.position {
            Some(self.val.cmp(&other.val))
        } else {
            None
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default_width = self.position.default_width();
        let width = f.width().unwrap_or(default_width);
        let mut v = self.val;

        // Drop digits from the right if we are showing msec and need a shorter width than usual.
        if self.is_msecs() && width < default_width {
            v = truncate_digits(v, default_width - width);
        }

        write!(f, "{:0>width$}", v, width = width)
    }
}

fn truncate_digits(v: u16, to_drop: usize) -> u16 {
    v / (10_u16).saturating_pow(to_drop.try_into().unwrap_or(1))
}

impl Field {
    /// Creates a field at `position` with value zero.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use zombiesplit::model::timing::time::{Field, Position};
    ///
    /// let f = Field::zero(Position::Seconds);
    /// assert_eq!(u16::from(f), 0);
    /// ```
    #[must_use]
    pub fn zero(position: Position) -> Self {
        Self { position, val: 0 }
    }

    /// Tries to construct a field with a given position and value, failing if the value is too big.
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use zombiesplit::model::timing::time::{Field, Position};
    ///
    /// let f1 = Field::new(Position::Seconds, 4);
    /// assert!(f1.is_ok(), "shouldn't have overflowed");
    /// let f2 = Field::new(Position::Seconds, 64);
    /// assert!(!f2.is_ok(), "should have overflowed");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::FieldTooBig` if `val` is too large for the field.
    pub fn new(position: Position, val: u32) -> Result<Self> {
        Carry::new(position, val).try_into()
    }

    /// Tries to construct a field with a given position, parsing it from an undelimited string.
    ///
    /// ```
    /// use zombiesplit::model::timing::time::{Field, Position};
    ///
    /// let f1 = Field::parse(Position::Seconds, "4");
    /// assert_eq!(Field::new(Position::Seconds, 4), f1);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns any errors from parsing the value (typically wrapped integer parsing errors) or
    /// from `new`.
    pub fn parse(position: Position, s: &str) -> Result<Self> {
        let val = if s.is_empty() {
            Ok(0)
        } else {
            position.parse_value(s)
        }?;
        Self::new(position, val)
    }

    /// Tries to find `position`'s delimiter in `s`, and parses the
    /// delimited number if one exists; otherwise, returns an empty field.
    ///
    /// # Errors
    ///
    /// Returns any errors from parsing the value (typically wrapped integer parsing errors) or
    /// from `new`.
    pub(super) fn parse_delimited(position: Position, s: &str) -> Result<(Self, &str)> {
        let (to_parse, to_return) = position.split_delimiter(s);
        Ok((Self::parse(position, to_parse)?, to_return))
    }

    /// Formats the value `v` in a way appropriate for this position.
    ///
    /// # Errors
    ///
    /// Returns various `fmt::Error` errors if formatting the value or its
    /// delimiter fails.
    pub(super) fn fmt_value(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nd = self.position.default_width();

        let width = f.width().unwrap_or(nd);

        let v = if self.is_msecs() && width < nd {
            truncate_digits(self.val, width - nd)
        } else {
            self.val
        };

        write!(f, "{:0>width$}", v, width = width)
    }

    /// Formats the value `v` with a delimiter, if nonzero.
    pub(super) fn fmt_value_delimited(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_msecs() {
            // No delimiter, always format
            self.fmt_value(f)?;
        } else if self.val != 0 {
            self.fmt_value(f)?;
            f.write_char(self.position.delimiter())?;
        }

        Ok(())
    }

    /// Returns this field's value as milliseconds.
    ///
    /// ```
    /// use zombiesplit::model::timing::time::{Field, Position};
    /// use std::convert::TryFrom;
    /// let msec = Field::new(Position::Seconds, 20).unwrap().as_msecs();
    /// assert_eq!(msec, 20_000, "20 secs = 20,000 msecs");
    /// ```
    #[must_use]
    pub fn as_msecs(self) -> u32 {
        u32::from(self.val) * self.position.ms_offset()
    }

    fn is_msecs(self) -> bool {
        matches!(self.position, Position::Milliseconds)
    }
}

/// A field with carry into the next position.
pub struct Carry {
    /// The field, modulo its position's capacity.
    pub field: Field,
    /// Any carry into the next position.
    pub carry: u32,
    /// The original value, stored for error reporting.
    pub original: u32,
}

impl Carry {
    /// Constructs a field at `position` with `val` modulo `position`'s cap.
    /// The remainder is stored as carry.
    #[must_use]
    pub fn new(position: Position, val: u32) -> Self {
        let divisor = position.cap();
        let carried = val % divisor;
        Self {
            field: Field {
                position,
                val: carried.try_into().expect("all caps should be <u16"),
            },
            carry: (val - carried) / divisor,
            original: val,
        }
    }
}

/// We can convert from a carry to a field, if the carry is empty.
impl TryFrom<Carry> for Field {
    type Error = Error;

    fn try_from(c: Carry) -> Result<Field> {
        if c.carry == 0 {
            Ok(c.field)
        } else {
            Err(Error::FieldTooBig {
                pos: c.field.position,
                val: c.original,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the unusual parsing behaviour of milliseconds works properly.
    mod msec {
        use super::*;

        /// Tests a millisecond parse.
        fn test_parse(from: &'static str, want: u16) {
            let t = Field::parse(Position::Milliseconds, from).expect("should be valid");
            assert_eq!(u16::from(t), want);
        }

        /// Tests a millisecond display.
        fn test_display(from: u16, want: &'static str) {
            let t = Field {
                position: Position::Milliseconds,
                val: from,
            };
            assert_eq!(format!("{t}"), want);
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
            let t = Field {
                val: 123,
                position: Position::Milliseconds,
            };
            assert_eq!(format!("{t:2}"), "12");
        }

        /// Tests that stretching a millisecond field to four digits zero-pads on the left.
        #[test]
        fn display_three_digits_as_four() {
            let t: Field = Field {
                position: Position::Milliseconds,
                val: 123,
            };
            assert_eq!(format!("{t:4}"), "0123");
        }
    }
}
