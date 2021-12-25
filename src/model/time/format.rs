/*! User-suppliable formatting for times.

These structures and related support let users tell zombiesplit how to lay out times on the UI. */

use crate::model::time;
use itertools::Itertools;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// Format configuration for times.
///
/// This conceptually takes the form of a list of (position, digit length) pairs, for instance
/// (hour, 3).
#[derive(DeserializeFromStr, SerializeDisplay, Clone, Debug)]
pub struct Format(Vec<Position>);

impl Format {
    /// Iterates over the position layout details in this time layout.
    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.0.iter()
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.chars()
            .group_by(|x| *x)
            .into_iter()
            .map(|(c, group)| parse_run(c, group.count()))
            .try_collect()
            .map(Format)
    }
}

fn parse_run(c: char, num_digits: usize) -> Result<Position> {
    Ok(Position {
        index: parse_char(c)?,
        num_digits,
    })
}

/// Time layouts can be displayed, in the same format as they are parsed.
impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for Position { index, num_digits } in &self.0 {
            f.write_str(&String::from(emit_char(*index)).repeat(*num_digits))?;
        }
        Ok(())
    }
}

/// Time parsing errors.
#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("Unexpected character: {0}")]
    BadChar(char),
}

/// Shorthand for results over time parsing.
pub type Result<T> = std::result::Result<T, Error>;

fn parse_char(c: char) -> Result<time::Position> {
    match c {
        CHAR_HOUR => Ok(time::Position::Hours),
        CHAR_MIN => Ok(time::Position::Minutes),
        CHAR_SEC => Ok(time::Position::Seconds),
        CHAR_MSEC => Ok(time::Position::Milliseconds),
        _ => Err(Error::BadChar(c)),
    }
}

fn emit_char(i: time::Position) -> char {
    match i {
        time::Position::Hours => CHAR_HOUR,
        time::Position::Minutes => CHAR_MIN,
        time::Position::Seconds => CHAR_SEC,
        time::Position::Milliseconds => CHAR_MSEC,
    }
}

const CHAR_HOUR: char = 'h';
const CHAR_MIN: char = 'm';
const CHAR_SEC: char = 's';
const CHAR_MSEC: char = 'u';

/// Layout information for one position index in a time layout.
///
/// A vector of these structures fully defines how the UI should render times.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    /// The index being displayed.
    pub index: time::Position,
    /// The number of digits to display for this index.
    pub num_digits: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests parsing a 2-minute/2-second/3-millisecond layout.
    #[test]
    fn test_time_parse_empty() {
        let actual: Format = "".parse().expect("parse failure");
        assert_eq!(0, actual.positions().count());
    }

    /// Tests parsing a 2-minute/2-second/3-millisecond layout.
    #[test]
    fn test_time_parse_mmssuuu() {
        let expected = vec![
            Position {
                index: time::Position::Minutes,
                num_digits: 2,
            },
            Position {
                index: time::Position::Seconds,
                num_digits: 2,
            },
            Position {
                index: time::Position::Milliseconds,
                num_digits: 3,
            },
        ];

        let actual: Format = "mmssuuu".parse().expect("parse failure");
        let apos: Vec<Position> = actual.positions().cloned().collect();
        assert_eq!(expected, apos);
    }

    /// Tests that round-tripping the parse/emit for index characters works ok.
    #[test]
    fn test_parse_char_round_trip() {
        use time::Position;

        let positions = [
            Position::Hours,
            Position::Minutes,
            Position::Seconds,
            Position::Milliseconds,
        ];

        for i in positions {
            assert_eq!(i, parse_char(emit_char(i)).expect("parse failure"));
        }
    }
}
