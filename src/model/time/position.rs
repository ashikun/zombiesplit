//! Position-specific field logic.
use std::{
    borrow::Cow,
    fmt::{self, Display, Write},
};

const MINS_IN_HOUR: u32 = 60;
const SECS_IN_MIN: u32 = 60;
const MSECS_IN_SEC: u32 = 1000;

/// Trait for position phantom types.
pub trait Position {
    /// The delimiter for this position.
    fn delimiter() -> char;

    /// The name of the position.
    fn name() -> Name;

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
        write!(f, "{:0>2}", v)
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
impl Position for Hour {
    fn delimiter() -> char {
        'h'
    }

    fn name() -> Name {
        Name::Hours
    }

    fn cap() -> u32 {
        u32::MAX
    }

    fn ms_offset() -> u32 {
        Minute::cap() * Minute::ms_offset()
    }
}

/// Phantom type for minutes.
pub struct Minute;
impl Position for Minute {
    fn delimiter() -> char {
        'm'
    }

    fn name() -> Name {
        Name::Minutes
    }

    fn cap() -> u32 {
        MINS_IN_HOUR
    }

    fn ms_offset() -> u32 {
        Second::cap() * Second::ms_offset()
    }
}

/// Phantom type for seconds.
pub struct Second;
impl Position for Second {
    fn delimiter() -> char {
        's'
    }

    fn name() -> Name {
        Name::Seconds
    }

    fn cap() -> u32 {
        SECS_IN_MIN
    }

    fn ms_offset() -> u32 {
        Msec::cap() * Msec::ms_offset()
    }
}

/// Phantom type for milliseconds.
///
/// Milliseconds have a subtly different parse/format from the other components.
pub struct Msec;
impl Position for Msec {
    fn delimiter() -> char {
        '\0'
    }

    fn name() -> Name {
        Name::Milliseconds
    }

    fn cap() -> u32 {
        MSECS_IN_SEC
    }

    fn ms_offset() -> u32 {
        1
    }

    // Milliseconds have a lot of parsing and formatting overrides, because
    // we treat them as being on the right hand side of a decimal place after
    // seconds.

    fn fmt_value(v: u16, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>3}", v)
    }

    fn fmt_value_delimited(v: u16, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::fmt_value(v, f)
    }

    /// Pads any millisecond string such that, for instance, 5 parses as 500ms.
    fn preprocess_string(s: &str) -> Cow<str> {
        if s.len() >= 3 {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(format!("{:0<3}", s))
        }
    }

    /// Milliseconds have no delimiter, so we just take the entire string.
    fn split_delimiter(s: &str) -> (&str, &str) {
        (s, "")
    }
}

/// Names of parseable time fields.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Name {
    /// Denotes the hours field.
    Hours,
    /// Denotes the minutes field.
    Minutes,
    /// Denotes the seconds field.
    Seconds,
    /// Denotes the milliseconds field.
    Milliseconds,
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Hours => "hours",
                Self::Minutes => "minutes",
                Self::Seconds => "seconds",
                Self::Milliseconds => "msececonds",
            }
        )
    }
}
