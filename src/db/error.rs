//! Database error types.

use thiserror::Error;

/// Database errors.
#[derive(Debug, Error)]
pub enum Error {
    /// A wrapped SQLite error.
    #[error("sqlite error: {0}")]
    SQLite(#[from] rusqlite::Error),

    /// A category referenced a segment not inserted in the database yet.
    #[error("couldn't find segment {short} requested by category {in_category}")]
    MissingSegment { short: String, in_category: String },

    /// A segment referenced a split not inserted in the database yet.
    #[error("couldn't find split {short} requested by segment {in_segment}")]
    MissingSplit { short: String, in_segment: String },

    /// A run referenced a split not inserted in the database yet.
    #[error("couldn't find split {short} requested by run")]
    MissingRunSplit { short: String },

    #[error("bad run timestamp: {0}")]
    BadRunTimestamp(i64),
}

/// Shorthand for a result over [Error].
pub type Result<T> = std::result::Result<T, Error>;
