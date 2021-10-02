//! View configuration.

use serde::{Deserialize, Serialize};

use super::gfx::{colour, font, metrics};

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'p> {
    /// Colour set.
    pub colours: colour::Set,
    /// Font configuration.
    #[serde(borrow)]
    pub fonts: font::Map<font::map::Path<'p>>,
    /// Window metrics.
    pub window: metrics::Window,
}
