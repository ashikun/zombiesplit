//! Errors for the zombiesplit graphics library.
use thiserror::Error;

/// A user interface error.
#[derive(Debug, Error)]
pub enum Error {
    /// A formatting error has occurred.
    ///
    /// This generally comes from trying to write to the UI using the
    /// `std::fmt::Write` trait.
    #[error("formatting error")]
    Fmt(#[from] std::fmt::Error),

    /// An error occurred while handling a font.
    #[error("font error: {0}")]
    LoadFont(#[from] super::font::Error),

    /// An error occurred while blitting the font.
    #[error("SDL couldn't blit font: {0}")]
    Blit(String),
}

/// Shorthand for a `std::result::Result` over [Error].
pub type Result<T> = std::result::Result<T, Error>;
