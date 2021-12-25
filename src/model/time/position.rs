//! Position-specific field logic.
use std::{
    borrow::Cow,
    fmt::{self, Display, Write},
};

const MINS_IN_HOUR: u32 = 60;
const SECS_IN_MIN: u32 = 60;
const MSECS_IN_SEC: u32 = 1000;

/// Trait for position phantom types.
pub trait Marker {
    /// The delimiter for this position.
    fn delimiter() -> char;

    /// The position for which this is a marker.
    fn position() -> Position;

    /// The multiplier needed to convert this position to milliseconds.
    fn ms_offset() -> u32;

    /// The cap for this field (maximum+1).
    fn cap() -> u32;

    /// Formats the value `v` in a way appropriate for this position.
    ///
    /// # Errors
    ///
    /// Returns various `fmt::Error` errors if formatting the value or its
    /// delimiter fails.
    fn fmt_value(v: u16, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(2);
        write!(f, "{:0>width$}", v, width = width)
    }

    /// Formats the value `v` with a delimiter, if nonzero.
    ///
    /// # Errors
    ///
    /// Returns various `fmt::Error` errors if formatting the value or its
    /// delimiter fails.
    fn fmt_value_delimited(v: u16, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if v != 0 {
            Self::fmt_value(v, f)?;
            let d = Self::delimiter();
            f.write_char(d)?;
        };
        Ok(())
    }

    /// Performs any preprocessing that should be done to a string before
    /// parsing it as a field.
    #[must_use]
    fn preprocess_string(s: &str) -> Cow<str> {
        Cow::Borrowed(s)
    }

    /// Splits `s` into a part before this position's delimiter (if any), and
    /// one after this position's delimiter.
    #[must_use]
    fn split_delimiter(s: &str) -> (&str, &str) {
        s.split_once(Self::delimiter()).unwrap_or(("", s))
    }
}

/// Phantom type for hours.
pub struct Hour;
impl Marker for Hour {
    fn delimiter() -> char {
        'h'
    }

    fn position() -> Position {
        Position::Hours
    }

    fn ms_offset() -> u32 {
        Minute::cap() * Minute::ms_offset()
    }

    fn cap() -> u32 {
        u32::MAX
    }
}

/// Phantom type for minutes.
pub struct Minute;
impl Marker for Minute {
    fn delimiter() -> char {
        'm'
    }

    fn position() -> Position {
        Position::Minutes
    }

    fn ms_offset() -> u32 {
        Second::cap() * Second::ms_offset()
    }

    fn cap() -> u32 {
        MINS_IN_HOUR
    }
}

/// Phantom type for seconds.
pub struct Second;
impl Marker for Second {
    fn delimiter() -> char {
        's'
    }

    fn position() -> Position {
        Position::Seconds
    }

    fn ms_offset() -> u32 {
        Msec::cap() * Msec::ms_offset()
    }

    fn cap() -> u32 {
        SECS_IN_MIN
    }
}

/// Phantom type for milliseconds.
///
/// Milliseconds have a subtly different parse/format from the other components.
pub struct Msec;
impl Marker for Msec {
    fn delimiter() -> char {
        '\0'
    }

    fn position() -> Position {
        Position::Milliseconds
    }

    fn ms_offset() -> u32 {
        1
    }

    fn cap() -> u32 {
        MSECS_IN_SEC
    }

    // Milliseconds have a lot of parsing and formatting overrides, because
    // we treat them as being on the right hand side of a decimal place after
    // seconds.

    fn fmt_value(mut v: u16, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = f.width().unwrap_or(MSEC_DIGITS);
        if width < MSEC_DIGITS {
            v = truncate_digits(v, width);
        }
        write!(f, "{:0>width$}", v, width = width)
    }

    fn fmt_value_delimited(v: u16, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::fmt_value(v, f)
    }

    /// Pads any millisecond string such that, for instance, 5 parses as 500ms.
    fn preprocess_string(s: &str) -> Cow<str> {
        if s.len() >= MSEC_DIGITS {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(format!("{:0<digits$}", s, digits = MSEC_DIGITS))
        }
    }

    /// Milliseconds have no delimiter, so we just take the entire string.
    fn split_delimiter(s: &str) -> (&str, &str) {
        (s, "")
    }
}

fn truncate_digits(v: u16, width: usize) -> u16 {
    let exp: u32 = (MSEC_DIGITS - width).try_into().unwrap_or(1);
    v / (10_u16).saturating_pow(exp)
}

const MSEC_DIGITS: usize = 3;

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Hours => "hours",
            Self::Minutes => "minutes",
            Self::Seconds => "seconds",
            Self::Milliseconds => "milliseconds",
        })
    }
}
