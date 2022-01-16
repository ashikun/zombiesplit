//! View configuration.

use serde::{Deserialize, Serialize};

use crate::model::timing::time;

use super::gfx::{colour, font, metrics};

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

/// Layout configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Layout {
    /// Window metrics.
    pub window: metrics::Window,
    /// Default format for times.
    pub time: time::Format,
}
