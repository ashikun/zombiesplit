/*! Time layout configuration.

These structures and related support tell zombiesplit how to lay out times on the UI. */

use crate::model::time::position;
use itertools::Itertools;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// Layout configuration for times.
///
/// This conceptually takes the form of a list of (position, digit length) pairs, for instance
/// (hour, 3).
#[derive(DeserializeFromStr, SerializeDisplay, Clone, Debug)]
pub struct Time(Vec<Position>);

impl Time {
    /// Iterates over the position layout details in this time layout.
    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.0.iter()
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.chars()
            .group_by(|x| *x)
            .into_iter()
            .map(|(c, group)| parse_run(c, group.count()))
            .try_collect()
            .map(Time)
    }
}

fn parse_run(c: char, num_digits: usize) -> Result<Position> {
    Ok(Position {
        index: parse_char(c)?,
        num_digits,
    })
}

/// Time layouts can be displayed, in the same format as they are parsed.
impl Display for Time {
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

fn parse_char(c: char) -> Result<position::Index> {
    match c {
        CHAR_HOUR => Ok(position::Index::Hours),
        CHAR_MIN => Ok(position::Index::Minutes),
        CHAR_SEC => Ok(position::Index::Seconds),
        CHAR_MSEC => Ok(position::Index::Milliseconds),
        _ => Err(Error::BadChar(c)),
    }
}

fn emit_char(i: position::Index) -> char {
    match i {
        position::Index::Hours => CHAR_HOUR,
        position::Index::Minutes => CHAR_MIN,
        position::Index::Seconds => CHAR_SEC,
        position::Index::Milliseconds => CHAR_MSEC,
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
    pub index: position::Index,
    /// The number of digits to display for this index.
    pub num_digits: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests parsing a 2-minute/2-second/3-millisecond layout.
    #[test]
    fn test_time_parse_empty() {
        let actual: Time = "".parse().expect("parse failure");
        assert_eq!(0, actual.positions().count());
    }

    /// Tests parsing a 2-minute/2-second/3-millisecond layout.
    #[test]
    fn test_time_parse_mmssuuu() {
        use position::Index;

        let expected = vec![
            Position {
                index: Index::Minutes,
                num_digits: 2,
            },
            Position {
                index: Index::Seconds,
                num_digits: 2,
            },
            Position {
                index: Index::Milliseconds,
                num_digits: 3,
            },
        ];

        let actual: Time = "mmssuuu".parse().expect("parse failure");
        let apos: Vec<Position> = actual.positions().cloned().collect();
        assert_eq!(expected, apos);
    }

    /// Tests that round-tripping the parse/emit for index characters works ok.
    #[test]
    fn test_parse_char_round_trip() {
        use position::Index;

        let positions = [
            Index::Hours,
            Index::Minutes,
            Index::Seconds,
            Index::Milliseconds,
        ];

        for i in positions {
            assert_eq!(i, parse_char(emit_char(i)).expect("parse failure"));
        }
    }
}
