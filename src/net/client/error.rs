//! Error types for the client.
use thiserror::Error;

/// Type of client errors.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error.
    #[error("i/o error")]
    Io(#[from] std::io::Error),
}

/// Shorthand for results over [Error].
pub type Result<T> = std::result::Result<T, Error>;
