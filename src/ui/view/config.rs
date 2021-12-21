//! View configuration.

pub mod time;

use serde::{Deserialize, Serialize};

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
    /// Information on how to lay-out times.
    pub time: time::Time,
}
