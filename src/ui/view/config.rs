//! View configuration.

use serde::{Deserialize, Serialize};

use super::gfx::{colour, font};

pub mod layout;
pub use layout::Layout;

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Config {
    /// Theme configuration.
    pub theme: Theme,
    /// Layout configuration.
    pub layout: Layout,
}

/// Theme configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Theme {
    /// Colour set.
    pub colours: colour::Set,
    /// Font configuration.
    pub fonts: font::Map<font::map::Path>,
}
