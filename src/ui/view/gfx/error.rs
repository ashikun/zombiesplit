//! Errors for the zombiesplit graphics library.
use thiserror::Error;

/// A user interface error.
#[derive(Debug, Error)]
pub enum Error {
    /// An I/O error has occurred.
    ///
    /// This generally comes from trying to write to the UI using the
    /// `std::io::Write` trait, and one might expect a more specific error to
    /// be boxed inside this error.
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    /// An error occurred while handling a font.
    #[error("font error: {0}")]
    LoadFont(#[from] super::font::Error),

    /// An error occurred while blitting the font.
    #[error("SDL couldn't blit font: {0}")]
    Blit(String),
}

/// Shorthand for a `std::result::Result` over [Error].
pub type Result<T> = std::result::Result<T, Error>;
