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

impl From<Colour> for sdl2::pixels::Color {
    fn from(c: Colour) -> Self {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let a = (255.0 * c.0.a).round() as u8;
        Self::RGBA(c.0.r, c.0.g, c.0.b, a)
    }
}
