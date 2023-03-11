//! Position-specific field logic.
use super::error::{Error, Result};
use num_integer::Integer;
use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter, Write},
    num::ParseIntError,
};

/// Enumeration of possible positions in a time.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Position {
    /// Denotes the hours field.
    Hours,
    /// Denotes the minutes field.
    Minutes,
    /// Denotes the seconds field.
    Seconds,
    /// Denotes the milliseconds field.
    Milliseconds,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Hours => "hours",
            Self::Minutes => "minutes",
            Self::Seconds => "seconds",
            Self::Milliseconds => "milliseconds",
        })
    }
}

//
// Conversion ratios between positions
//

const MINS_IN_HOUR: u16 = 60;
const SECS_IN_MIN: u16 = 60;
const MSECS_IN_SEC: u16 = 1_000;

impl Position {
    /// Slice containing every position in decreasing order of significance.
    pub(super) const ALL: &'static [Self] = &[
        Self::Hours,
        Self::Minutes,
        Self::Seconds,
        Self::Milliseconds,
    ];

    /// The delimiter for this position.
    pub(super) const fn delimiter(self) -> Option<char> {
        match self {
            Self::Hours => Some('h'),
            Self::Minutes => Some('m'),
            Self::Seconds => Some('s'),
            Self::Milliseconds => None,
        }
    }

    /// The multiplier needed to convert this position to milliseconds.
    pub(super) const fn ms_offset(self) -> u16 {
        match self {
            Self::Hours => MINS_IN_HOUR * SECS_IN_MIN * MSECS_IN_SEC,
            Self::Minutes => SECS_IN_MIN * MSECS_IN_SEC,
            Self::Seconds => MSECS_IN_SEC,
            Self::Milliseconds => 1,
        }
    }

    /// The number of digits displayed or parsed, by default, for this position.
    pub(super) const fn default_width(self) -> usize {
        match self {
            Self::Milliseconds => 3,
            _ => 2,
        }
    }

    /// The amount (maximum+1) at which this field will overflow to the next.
    pub(super) const fn capacity(self) -> u16 {
        match self {
            Self::Hours => u16::MAX,
            Self::Minutes => MINS_IN_HOUR,
            Self::Seconds => SECS_IN_MIN,
            Self::Milliseconds => MSECS_IN_SEC,
        }
    }

    /// Splits a string on this field's delimiter, parses the first part of the string as a
    /// value at this particular field, and passes through the second part for further parsing.
    pub(super) fn split_and_parse(self, s: &str) -> Result<(u16, &str)> {
        let (before, after) = self.split_delimiter(s);
        self.preprocess_string(before)
            .parse()
            .map_err(|err: ParseIntError| Error::FieldParse { pos: self, err })
            .map(|v| (v, after))
    }

    /// Performs any preprocessing that should be done to a string before
    /// parsing it as a field.
    ///
    /// For milliseconds, this involves left-padding it.
    #[must_use]
    fn preprocess_string(self, s: &str) -> Cow<str> {
        let digits = self.default_width();
        if matches!(self, Self::Milliseconds) && s.len() < digits {
            Cow::Owned(format!("{s:0<digits$}"))
        } else {
            Cow::Borrowed(s)
        }
    }

    /// Splits `s` into a part before this position's delimiter (if any), and
    /// one after this position's delimiter.
    #[must_use]
    pub(super) fn split_delimiter(self, s: &str) -> (&str, &str) {
        match self.delimiter() {
            None => (s, ""),
            Some(d) => s.split_once(d).unwrap_or(("", s)),
        }
    }

    /// Formats the value `v` in a way appropriate for this position.
    ///
    /// # Errors
    ///
    /// Returns various `fmt::Error` errors if formatting the value or its
    /// delimiter fails.
    pub(super) fn fmt_value(self, f: &mut Formatter<'_>, mut v: u16) -> fmt::Result {
        let nd = self.default_width();
        let width = f.width().unwrap_or(nd);

        // Truncate any unneeded rightmost zeroes from a milliseconds display.
        if self == Self::Milliseconds && width < nd {
            let to_drop = width - nd;
            v /= (10_u16).saturating_pow(to_drop.try_into().unwrap_or(1))
        }

        write!(f, "{:0>width$}", v, width = width)
    }

    /// Formats the value `v` with a delimiter, if nonzero.
    pub(super) fn fmt_value_delimited(self, f: &mut Formatter<'_>, v: u16) -> fmt::Result {
        if let Some(d) = self.delimiter() {
            // Only show a field with a delimiter if it is nonzero.
            if v != 0 {
                self.fmt_value(f, v)?;
                f.write_char(d)?;
            }
        } else {
            // No delimiter, always format
            self.fmt_value(f, v)?;
        }
        Ok(())
    }

    /// Tries to fit `input` into this position, returning the result as well as any carry.
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::time::position::{Position, Fit, Value};
    ///
    /// // 12345 milliseconds is 12 seconds 345 milliseconds
    /// assert_eq!(
    ///     Fit {
    ///         input: 1234,
    ///         result: Value {field: Position::Milliseconds, value: 345},
    ///         overflow: 12
    ///     },
    ///     Position::Milliseconds::fit(12345)
    /// );
    /// // 320 seconds is 5 minutes 20 seconds
    /// assert_eq!(
    ///     Fit {
    ///         input: 320,
    ///         result: Value {field: Position::Seconds, value: 20},
    ///         overflow: 5
    ///     },
    ///     Position::Seconds::fit(320)
    /// );
    /// ```
    pub(super) fn fit(self, input: u32) -> Fit {
        let divisor = u32::from(self.capacity());
        let (overflow, rem) = input.div_rem(&divisor);
        let value = rem.try_into().expect("position overflows should be <u16");
        let result = Value { field: self, value };
        Fit {
            result,
            overflow,
            input,
        }
    }
}

/// The result of a call to `Position::fit`.
pub struct Fit {
    /// The result (input modulo the capacity of the field).
    pub result: Value,
    /// Any overflow into the next position.
    pub overflow: u32,
    /// The original value, stored for error reporting.
    pub input: u32,
}

/// A combination of a field and value.
///
/// There is no check to make sure that `value` is below the `overflow` of `field`.
pub struct Value {
    /// The field to which this value belongs.
    pub field: Position,
    /// The value assigned to the field.
    pub value: u16,
}

/// Values can be displayed (and will show their delimiter).
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.field.fmt_value_delimited(f, self.value)
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
            let (t, rest) = Position::Milliseconds
                .split_and_parse(from)
                .expect("should be valid");
            assert_eq!(u16::from(t), want);
            // We should consume the whole string.
            assert_eq!("", rest);
        }

        /// Tests a millisecond display.
        fn test_display(from: u16, want: &'static str) {
            let mut buf = String::new();
            let t = Value {
                field: Position::Milliseconds,
                value: from,
            };
            assert_eq!(want, t.to_string());
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
