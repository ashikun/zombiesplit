//! Database error types.

use thiserror::Error;

/// Database errors.
#[derive(Debug, Error)]
pub enum Error {
    /// A wrapped SQLite error.
    #[error("SQLite error: {0}")]
    SQLite(#[from] rusqlite::Error),
}

/// Shorthand for a result over [Error].
pub type Result<T> = std::result::Result<T, Error>;
