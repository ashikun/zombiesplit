/*! User-suppliable formatting for times.

These structures and related support let users tell zombiesplit how to lay out times on the UI. */

use crate::model::time;
use crate::model::time::Position;
use itertools::Itertools;
use num_integer::Integer;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};
use thiserror::Error;

/// Format configuration for times.
///
/// This conceptually takes the form of a list of (position, digit length) pairs, for instance
/// (hour, 3).
#[derive(DeserializeFromStr, SerializeDisplay, Clone, Debug)]
pub struct Format(Vec<Component>);

/// The default format is mm'ss"uuu.
impl Default for Format {
    fn default() -> Self {
        Format(vec![
            Component::Position {
                position: Position::Minutes,
                width: 2,
            },
            Component::Delimiter('\''),
            Component::Position {
                position: Position::Seconds,
                width: 2,
            },
            Component::Delimiter('"'),
            Component::Position {
                position: Position::Milliseconds,
                width: 3,
            },
        ])
    }
}

impl Format {
    /// Iterates over the position layout details in this time layout.
    pub fn components(&self) -> impl Iterator<Item = &Component> {
        self.0.iter()
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Parser::default().parse(s).map(Self)
    }
}

#[derive(Default)]
struct Parser {
    /// Whether the parser just ate an escape character (and the next character is to be escaped).
    is_escaping: bool,
    /// The vector being built.
    result: Vec<Component>,
}

impl Parser {
    fn parse(mut self, s: &str) -> Result<Vec<Component>> {
        // TODO(@MattWindsor91): there might be a more efficient way of doing this
        for (mut count, c) in s.chars().dedup_with_count() {
            // Handle any escaped character first.
            if self.is_escaping {
                self.is_escaping = false;
                self.push_delimiter(c);
                count -= 1;
            }
            // Are there now further, unescaped, characters to go?
            if 0 < count {
                if c == CHAR_ESC {
                    // Pairs of consecutive escape characters are escaped escapes, so pass those
                    // through to the next part.
                    let (whole_escs, rem) = count.div_rem(&2_usize);
                    count = whole_escs;
                    self.is_escaping = rem != 0;
                }

                if let Some(index) = parse_position_char(c) {
                    self.push_position(count, index);
                } else {
                    (0..count).for_each(|_| self.push_delimiter(c));
                }
            }
        }

        if self.is_escaping {
            Err(Error::UnbalancedEscape)
        } else {
            Ok(self.result)
        }
    }

    fn push_position(&mut self, num_digits: usize, index: Position) {
        self.result.push(Component::Position {
            position: index,
            width: num_digits,
        });
    }

    fn push_delimiter(&mut self, c: char) {
        self.result.push(Component::Delimiter(c));
    }
}

fn parse_position_char(c: char) -> Option<time::Position> {
    match c {
        CHAR_HOUR => Some(time::Position::Hours),
        CHAR_MIN => Some(time::Position::Minutes),
        CHAR_SEC => Some(time::Position::Seconds),
        CHAR_MSEC => Some(time::Position::Milliseconds),
        _ => None,
    }
}

/// Time layouts can be displayed, in the same format as they are parsed.
impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            c.fmt(f)?;
        }
        Ok(())
    }
}

fn char_of_position(i: time::Position) -> char {
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
const CHAR_ESC: char = '\\';
const SPECIAL_CHARS: [char; 5] = [CHAR_HOUR, CHAR_MIN, CHAR_SEC, CHAR_MSEC, CHAR_ESC];

/// Layout information for one component in a time layout.
///
/// A vector of these structures fully defines how the UI should render times.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Component {
    /// A position component.
    Position {
        /// The position being displayed.
        position: time::Position,
        /// The number of digits to display for this index.
        width: usize,
    },
    /// A delimiter.
    Delimiter(char),
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Position { position, width } => emit_position(f, position, width),
            Self::Delimiter(c) => emit_delimiter(f, c),
        }
    }
}

fn emit_position(f: &mut Formatter, position: Position, width: usize) -> std::fmt::Result {
    let c = char_of_position(position);
    for _ in 0..width {
        f.write_char(c)?;
    }
    Ok(())
}

fn emit_delimiter(f: &mut Formatter, c: char) -> std::fmt::Result {
    if SPECIAL_CHARS.contains(&c) {
        f.write_char(CHAR_ESC)?;
    }
    f.write_char(c)
}

/// Time parsing errors.
#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("Expected a character after '\\'")]
    UnbalancedEscape,
}

/// Shorthand for results over time parsing.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Tests that the default format is mm'ss"uuu.
    #[test]
    fn test_default_mmssuuu() {
        assert_eq!("mm'ss\"uuu", Format::default().to_string());
    }

    /// Tests parsing a 2-minute/2-second/3-millisecond layout.
    #[test]
    fn test_time_parse_empty() {
        let actual: Format = "".parse().expect("parse failure");
        assert_eq!(0, actual.components().count());
    }

    /// Tests parsing a failed escape.
    #[test]
    fn test_time_parse_failed_escape() {
        let e = "\\"
            .parse::<Format>()
            .expect_err("should have failed to parse here");
        assert_eq!(Error::UnbalancedEscape, e);
    }

    /// Tests parsing a 2-minute/2-second/2-millisecond layout with a fairly contrived use of / as
    /// delimiter.
    #[test]
    fn test_time_parse_mmssuu_slashes() {
        let expected = vec![
            Component::Position {
                position: time::Position::Minutes,
                width: 2,
            },
            Component::Delimiter('\\'),
            Component::Position {
                position: time::Position::Seconds,
                width: 2,
            },
            Component::Delimiter('\\'),
            Component::Delimiter('\\'),
            Component::Position {
                position: time::Position::Milliseconds,
                width: 2,
            },
        ];

        let actual: Format = r"mm\\ss\\\\uu".parse().expect("parse failure");
        let apos: Vec<Component> = actual.components().cloned().collect();
        assert_eq!(expected, apos);
    }

    /// Tests parsing a 2-minute/2-second/3-millisecond layout.
    #[test]
    fn test_time_parse_mmssuuu() {
        let expected = vec![
            Component::Position {
                position: time::Position::Minutes,
                width: 2,
            },
            Component::Delimiter('"'),
            Component::Position {
                position: time::Position::Seconds,
                width: 2,
            },
            Component::Delimiter('\''),
            Component::Position {
                position: time::Position::Milliseconds,
                width: 3,
            },
        ];

        let actual: Format = "mm\"ss\'uuu".parse().expect("parse failure");
        let apos: Vec<Component> = actual.components().cloned().collect();
        assert_eq!(expected, apos);
    }

    /// Tests parsing a 2-hour/2-minute/2-second layout with 'h', 'm', and 's' delimiters.
    #[test]
    fn test_time_parse_hms_with_letters() {
        let expected = vec![
            Component::Position {
                position: time::Position::Hours,
                width: 2,
            },
            Component::Delimiter('h'),
            Component::Position {
                position: time::Position::Minutes,
                width: 2,
            },
            Component::Delimiter('m'),
            Component::Position {
                position: time::Position::Seconds,
                width: 2,
            },
            Component::Delimiter('s'),
        ];

        let actual: Format = r"hh\hmm\mss\s".parse().expect("parse failure");
        let apos: Vec<Component> = actual.components().cloned().collect();
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
            assert_eq!(
                i,
                parse_position_char(char_of_position(i)).expect("parse failure")
            );
        }
    }
}
