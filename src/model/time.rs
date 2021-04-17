//! ZombieSplit's notion of times.
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};
use thiserror::Error;

/// A hh:mm:ss:ms timing.
#[derive(SerializeDisplay, DeserializeFromStr, Debug)]
pub struct Time {
    /// Number of hours.
    pub hours: u8,
    /// Number of minutes.
    pub mins: u8,
    /// Number of seconds.
    pub secs: u8,
    /// Number of milliseconds.
    pub micros: u16,
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if 0 < self.hours {
            write!(f, "{}h", self.hours)?;
        }
        if 0 < self.mins {
            write!(f, "{}m", self.mins)?;
        }
        write!(f, "{}s", self.secs)?;
        if 0 < self.micros {
            write!(f, "{}", self.micros)?;
        }
        Ok(())
    }
}

/// Represents fields in the [Time] structure.
#[derive(Copy, Clone, Debug)]
pub enum Field {
    /// Denotes the hours field.
    Hours,
    /// Denotes the minutes field.
    Minutes,
    /// Denotes the seconds field.
    Seconds,
    /// Denotes the microseconds field.
    Micros,
}

impl Field {
    fn delimiter(&self) -> char {
        use Field::*;
        match self {
            Hours => 'h',
            Minutes => 'm',
            Seconds => 's',
            Micros => ' ',
        }
    }

    /// Retrives the maximum value of this field.
    fn max(&self) -> u16 {
        use Field::*;
        match self {
            Hours => u16::from(u8::MAX),
            Minutes => 59,
            Seconds => 59,
            Micros => 999,
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Field::*;
        write!(
            f,
            "{}",
            match self {
                Hours => "hours",
                Minutes => "minutes",
                Seconds => "seconds",
                Micros => "microseconds",
            }
        )
    }
}

/// An error that occurs when parsing a time.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("field {field} failed parsing: {err}")]
    FieldParseError { field: Field, err: ParseIntError },
    #[error("field {field} too big: was {val}, max {max}")]
    FieldTooBigError { field: Field, val: u16, max: u16 },
}

fn parse_inner<T: Copy + FromStr<Err = ParseIntError> + Into<u16>>(
    s: &str,
    field: Field,
) -> Result<T, ParseError> {
    s.parse()
        .map_err(|err| ParseError::FieldParseError { field, err })
        .and_then(|rval: T| {
            let max = field.max();
            let val = rval.into();
            if val <= max {
                Ok(rval)
            } else {
                Err(ParseError::FieldTooBigError { field, val, max })
            }
        })
}

fn parse_component(s: &str, field: Field) -> Result<(u8, &str), ParseError> {
    let d = field.delimiter();
    if let Some(ix) = s.find(d) {
        let (fst, s) = s.split_at(ix);
        let s = s.strip_prefix(d).unwrap_or(s);
        Ok((parse_inner(fst, field)?, s))
    } else {
        Ok((0, s))
    }
}

fn parse_micros(s: &str) -> Result<u16, ParseError> {
    if s.is_empty() {
        Ok(0)
    } else {
        parse_inner(s, Field::Micros)
    }
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hours, s) = parse_component(s, Field::Hours)?;
        let (mins, s) = parse_component(s, Field::Minutes)?;
        let (secs, s) = parse_component(s, Field::Seconds)?;
        let micros = parse_micros(s)?;
        Ok(Self {
            hours,
            mins,
            secs,
            micros,
        })
    }
}

mod tests {
    #[test]
    fn time_from_str_empty() {
        let t: super::Time = "".parse().expect("should be valid");
        assert_eq!(t.hours, 0);
        assert_eq!(t.mins, 0);
        assert_eq!(t.secs, 0);
        assert_eq!(t.micros, 0);
    }

    #[test]
    fn time_from_str_micros_only() {
        // This case may be removed later on, it's a bit weird.
        let t: super::Time = "123".parse().expect("should be valid");
        assert_eq!(t.hours, 0);
        assert_eq!(t.mins, 0);
        assert_eq!(t.secs, 0);
        assert_eq!(t.micros, 123);
    }

    #[test]
    fn time_from_str_secs_only() {
        let t: super::Time = "10s".parse().expect("should be valid");
        assert_eq!(t.hours, 0);
        assert_eq!(t.mins, 0);
        assert_eq!(t.secs, 10);
        assert_eq!(t.micros, 0);
    }

    #[test]
    fn time_from_str_secs_micros() {
        let t: super::Time = "10s500".parse().expect("should be valid");
        assert_eq!(t.hours, 0);
        assert_eq!(t.mins, 0);
        assert_eq!(t.secs, 10);
        assert_eq!(t.micros, 500);
    }

    #[test]
    fn time_from_str_all() {
        let t: super::Time = "1h2m3s456".parse().expect("should be valid");
        assert_eq!(t.hours, 1);
        assert_eq!(t.mins, 2);
        assert_eq!(t.secs, 3);
        assert_eq!(t.micros, 456);
    }
}
