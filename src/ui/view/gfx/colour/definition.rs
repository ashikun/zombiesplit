//! Colour definitions.

use super::error::{Error, Result};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, str::FromStr};

/// A colour.
#[derive(Copy, Clone, Debug, DeserializeFromStr, SerializeDisplay)]
pub struct Colour(css_color_parser::Color);

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Colour {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Colour(s.parse()?))
    }
}

impl Colour {
    /// Gets the red component of this colour as a byte.
    #[must_use]
    pub fn red_byte(&self) -> u8 {
        self.0.r
    }

    /// Gets the green component of this colour as a byte.
    #[must_use]
    pub fn green_byte(&self) -> u8 {
        self.0.g
    }

    /// Gets the blue component of this colour as a byte.
    #[must_use]
    pub fn blue_byte(&self) -> u8 {
        self.0.b
    }

    /// Gets the alpha component of this colour as a byte.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    #[must_use]
    pub fn alpha_byte(&self) -> u8 {
        (255.0 * self.0.a).round() as u8
    }
}
