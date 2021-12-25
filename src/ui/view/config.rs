//! View configuration.

use serde::{Deserialize, Serialize};

use crate::model::time;

use super::gfx::{colour, font, metrics};

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config<'p> {
    /// Colour set.
    pub colours: colour::Set,
    /// Font configuration.
    #[serde(borrow)]
    pub fonts: font::Map<font::map::Path<'p>>,
    /// Layout configuration.
    pub layout: Layout,
}

/// Layout configuration.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Layout {
    /// Window metrics.
    pub window: metrics::Window,
    /// Default format for times.
    pub time: time::Format,
}
