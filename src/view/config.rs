//! View configuration.

use serde::{Deserialize, Serialize};

use super::gfx::{colour, font, metrics};

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Colour set.
    pub colours: colour::Set,
    /// Font configuration.
    pub fonts: font::Set,
    /// Window metrics.
    pub window: metrics::Window,
}
