//! Decoding errors.

use crate::model::timing::time;
use thiserror::Error;

/// Shorthand for results over [Error].
pub type Result<T> = std::result::Result<T, Error>;

/// Decoding errors.
///
/// While this enumeration exists mainly for the benefit of client-side code, there is a mapping
/// into These map into gRPC status codes for use on the server side, but are enumerated at a
#[derive(Debug, Error, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A 64-bit index couldn't fit inside a `usize` (eg, we are on a 32-bit system).
    #[error("index conversion error")]
    IndexConversion(#[from] std::num::TryFromIntError),
    /// Couldn't decode a time.
    #[error("couldn't decode time")]
    Time(#[from] time::Error),
    /// Something we needed was missing.
    #[error("missing data in response: {0:?}")]
    Missing(Missing),
    /// An enumeration was out of range.
    #[error("out of range: {0:?}")]
    Unknown(Unknown),
}

/// We can express decoding errors as gRPC status codes, for use in server-side decoding.
impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        match e {
            Error::IndexConversion(_) => tonic::Status::out_of_range(e.to_string()),
            Error::Time(e) => adapt_time_error(e),
            Error::Missing(_) => tonic::Status::data_loss(e.to_string()),
            Error::Unknown(_) => tonic::Status::out_of_range(e.to_string()),
        }
    }
}

fn adapt_time_error(err: time::Error) -> tonic::Status {
    match err {
        time::Error::MsecOverflow(k) => {
            tonic::Status::out_of_range(format!("millisecond value {k} too large"))
        }
        _ => tonic::Status::invalid_argument(format!("invalid time: {err}")),
    }
}

/// Enumeration of missing-but-required data fields (including out-of-range enums).
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum Missing {
    /// The run was missing in a dump.
    Attempt,
    /// The attempt was missing its metadata.
    AttemptInfo,
    /// A split event was missing its payload.
    SplitEvent,
}

impl Missing {
    /// Lifts `val` to an error where, if `val` is missing, the result will be this [Missing] tag
    /// lifted to [Error].
    pub fn require<T>(self, val: Option<T>) -> Result<T> {
        val.ok_or(Error::Missing(self))
    }
}

/// Enumeration of missing-but-required data fields (including out-of-range enums).
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[non_exhaustive]
pub enum Unknown {
    /// The type was missing in a total event.
    TotalType,
    /// The pace was missing in an attempt total event.
    Pace,
    /// The comparison total type was missing in a comparison total event.
    ComparisonTotalType,
    /// The pop type was missing in a pop event or request.
    Pop,
}

impl Unknown {
    /// Lifts `val` to an error where, if `val` is missing, the result will be this [Unknown] tag
    /// lifted to [Error].
    pub fn require<T>(self, val: Option<T>) -> Result<T> {
        val.ok_or(Error::Unknown(self))
    }
}
