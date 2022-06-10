//! User interface errors.
use thiserror::Error;

/// A user interface error.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred in the graphics library.
    #[error("Graphical subsystem error: {0}")]
    Gfx(#[from] ugly::Error),

    /// An error occurred while initialising an SDL subsystem.
    #[error("SDL init error: {0}")]
    Init(String),

    /// An error occurred while building a window.
    #[error("SDL windowing error")]
    Window(#[from] sdl2::video::WindowBuildError),

    /// An error occurred while building a window.
    #[error("SDL error")]
    SdlInteger(#[from] sdl2::IntegerOrSdlError),

    /// An error occurred while formatting some text.
    #[error("Text formatting error")]
    Fmt(#[from] std::fmt::Error),
}

/// Shorthand for a result using [Error].
pub type Result<T> = std::result::Result<T, Error>;
