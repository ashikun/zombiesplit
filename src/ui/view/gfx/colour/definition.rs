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
    /// Constructs a colour using bytes for red, green, and blue components (and full alpha).
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::colour::definition::Colour;
    ///
    /// let col = Colour::rgb(12, 34, 56);
    /// assert_eq!(12, col.red_byte());
    /// assert_eq!(34, col.green_byte());
    /// assert_eq!(56, col.blue_byte());
    /// assert_eq!(255, col.alpha_byte());
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(css_color_parser::Color { r, g, b, a: 1.0 })
    }

    /// Gets the red component of this colour as a byte.
    #[must_use]
    pub const fn red_byte(&self) -> u8 {
        self.0.r
    }

    /// Gets the green component of this colour as a byte.
    #[must_use]
    pub const fn green_byte(&self) -> u8 {
        self.0.g
    }

    /// Gets the blue component of this colour as a byte.
    #[must_use]
    pub const fn blue_byte(&self) -> u8 {
        self.0.b
    }

    /// Gets the alpha component of this colour as a byte.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    #[must_use]
    pub fn alpha_byte(&self) -> u8 {
        (255.0 * self.0.a).round() as u8
    }
}
