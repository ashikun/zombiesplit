//! Low-level helpers for parsing times.

use std::num::ParseIntError;
use thiserror::Error;

/// An error that occurs when parsing a time.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("field {pos} failed parsing: {err}")]
    FieldParse {
        pos: super::Position,
        err: ParseIntError,
    },
    #[error("field {pos} too big: was {val}")]
    FieldTooBig { pos: super::Position, val: u32 },
    #[error("millisecond value {0} too large")]
    SecOverflow(i32),
    #[error("couldn't convert {0} to millisecond value; too large to fit")]
    MsecOverflow(super::human::Time),
}

/// Shorthand for parse results.
pub type Result<T> = std::result::Result<T, Error>;
