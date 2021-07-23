//! Errors raised by the font subsystem.
use thiserror::Error;

/// A font error.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred while loading the font.
    #[error("couldn't load font: {0}")]
    Load(String),

    /// We tried to configure a font using a nonexistent ID.
    #[error("font id not recognised: {0}")]
    Unknown(String),
}

/// Shorthand for a result using [Error].
pub type Result<T> = std::result::Result<T, Error>;
