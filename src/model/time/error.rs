//! Low-level helpers for parsing times.

use super::position;
use std::num::ParseIntError;
use thiserror::Error;

/// An error that occurs when parsing a time.
#[derive(Error, Debug)]
pub enum Error {
    #[error("field {pos} failed parsing: {err}")]
    FieldParse {
        pos: position::Index,
        err: ParseIntError,
    },
    #[error("field {pos} too big: was {val}")]
    FieldTooBig { pos: position::Index, val: u32 },
    #[error("millisecond value {0} too large")]
    MsecOverflow(u32),
}

/// Shorthand for parse results.
pub type Result<T> = std::result::Result<T, Error>;
