//! Position-specific field logic.
use super::error::{Error, Result};
use std::num::ParseIntError;
use std::{borrow::Cow, fmt};

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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Hours => "hours",
            Self::Minutes => "minutes",
            Self::Seconds => "seconds",
            Self::Milliseconds => "milliseconds",
        })
    }
}

impl Position {
    // This impl defines mostly constant lookup tables for positions.
    // We define anything that deals with a value on `Field`.

    /// The delimiter for this position.
    pub(super) const fn delimiter(self) -> char {
        match self {
            Position::Hours => 'h',
            Position::Minutes => 'm',
            Position::Seconds => 's',
            Position::Milliseconds => '\0',
        }
    }

    /// The multiplier needed to convert this position to milliseconds.
    pub(super) const fn ms_offset(self) -> u32 {
        match self {
            Position::Hours => MINS_IN_HOUR * SECS_IN_MIN * MSECS_IN_SEC,
            Position::Minutes => SECS_IN_MIN * MSECS_IN_SEC,
            Position::Seconds => MSECS_IN_SEC,
            Position::Milliseconds => 1,
        }
    }

    /// The number of digits displayed or parsed, by default, for this position.
    pub(super) const fn num_digits(self) -> usize {
        match self {
            Position::Milliseconds => 3,
            _ => 2,
        }
    }

    /// The cap for this field (maximum+1).
    pub(super) const fn cap(self) -> u32 {
        match self {
            Position::Hours => u16::MAX as u32,
            Position::Minutes => MINS_IN_HOUR,
            Position::Seconds => SECS_IN_MIN,
            Position::Milliseconds => MSECS_IN_SEC,
        }
    }

    pub(super) fn parse_value(self, s: &str) -> Result<u32> {
        self.preprocess_string(s)
            .parse()
            .map_err(|err: ParseIntError| Error::FieldParse { pos: self, err })
    }

    /// Performs any preprocessing that should be done to a string before
    /// parsing it as a field.
    ///
    /// For milliseconds, this involves left-padding it.
    #[must_use]
    fn preprocess_string(self, s: &str) -> Cow<str> {
        if matches!(self, Self::Milliseconds) && s.len() < self.num_digits() {
            Cow::Owned(format!("{:0<digits$}", s, digits = self.num_digits()))
        } else {
            Cow::Borrowed(s)
        }
    }

    /// Splits `s` into a part before this position's delimiter (if any), and
    /// one after this position's delimiter.
    #[must_use]
    pub(super) fn split_delimiter(self, s: &str) -> (&str, &str) {
        match self.delimiter() {
            '\0' => (s, ""),
            d => s.split_once(d).unwrap_or(("", s)),
        }
    }
}

const MINS_IN_HOUR: u32 = 60;
const SECS_IN_MIN: u32 = 60;
const MSECS_IN_SEC: u32 = 1000;
