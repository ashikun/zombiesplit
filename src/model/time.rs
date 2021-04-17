//! ZombieSplit's notion of times.
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};

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
    pub micros: u8,
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ParseError {
    pub field: Field,
    pub rest: ParseIntError,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad {} field", self.field) // for now
    }
}

fn parse_component<'a>(s: &'a str, field: Field) -> Result<(u8, &'a str), ParseError> {
    if let Some(ix) = s.find(field.delimiter()) {
        let (fst, s) = s.split_at(ix);
        Ok((fst.parse().map_err(|rest| ParseError { field, rest })?, s))
    } else {
        Ok((0, s))
    }
}

fn parse_micros(s: &str) -> Result<u8, ParseError> {
    if s.is_empty() {
        Ok(0)
    } else {
        s.parse().map_err(|rest| ParseError {
            field: Field::Micros,
            rest,
        })
    }
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hours, s) = parse_component(s, Field::Hours)?;
        let (mins, s) = parse_component(s, Field::Hours)?;
        let (secs, s) = parse_component(s, Field::Hours)?;
        let micros = parse_micros(s)?;
        Ok(Self {
            hours,
            mins,
            secs,
            micros,
        })
    }
}
