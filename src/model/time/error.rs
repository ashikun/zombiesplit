//! Low-level helpers for parsing times.

use std::num::ParseIntError;
use thiserror::Error;

/// An error that occurs when parsing a time.
#[derive(Error, Debug)]
pub enum Error {
    #[error("field {pos} failed parsing: {err}")]
    FieldParse {
        pos: super::Position,
        err: ParseIntError,
    },
    #[error("field {pos} too big: was {val}")]
    FieldTooBig { pos: super::Position, val: u32 },
    #[error("millisecond value {0} too large")]
    MsecOverflow(u32),
}

/// Shorthand for parse results.
pub type Result<T> = std::result::Result<T, Error>;
