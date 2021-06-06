//! Colour lookup errors.

use thiserror::Error;

/// Errors that can occur when parsing a colour.
#[derive(Debug, Error)]
pub enum Error {
    #[error("malformed colour")]
    Malformed(#[from] css_color_parser::ColorParseError),
}

/// Shorthand for result type.
pub type Result<T> = std::result::Result<T, Error>;
