//! Database error types.

use thiserror::Error;

/// Database errors.
#[derive(Debug, Error)]
pub enum Error {
    /// A wrapped SQLite error.
    #[error("SQLite error: {0}")]
    SQLite(#[from] rusqlite::Error),

    /// Couldn't find a primary key for something that should have been added
    /// to the database.
    #[error("Internal error: can't find primary key for {0}")]
    NoPrimaryKey(String),
}

/// Shorthand for a result over [Error].
pub type Result<T> = std::result::Result<T, Error>;
