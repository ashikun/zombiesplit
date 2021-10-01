//! View configuration.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::gfx::{colour, font, metrics};

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'p> {
    /// Colour set.
    pub colours: colour::Set,
    /// Font configuration.
    #[serde(borrow)]
    pub fonts: font::Map<&'p Path>,
    /// Window metrics.
    pub window: metrics::Window,
}
