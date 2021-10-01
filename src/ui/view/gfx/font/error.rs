//! Errors raised by the font subsystem.
use thiserror::Error;

/// A font error.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred while loading a font file.
    #[error("IO error reading font file")]
    Io(#[from] std::io::Error),

    /// An error occurred while loading a metrics TOML file.
    #[error("Error parsing metrics file from TOML")]
    Toml(#[from] toml::de::Error),

    /// Error loading a texture file.
    #[error("Error loading font texture")]
    TextureLoad(String),

    /// We tried to configure a font using a nonexistent ID.
    #[error("font id not recognised: {0}")]
    Unknown(String),
}

/// Shorthand for a result using [Error].
pub type Result<T> = std::result::Result<T, Error>;
