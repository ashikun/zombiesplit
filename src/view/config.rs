//! View configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::gfx::{colour, font, metrics};

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Colour set.
    pub colours: colour::Set,
    /// Font configuration.
    pub fonts: HashMap<font::Id, font::Config>,
    /// Window metrics.
    pub window: metrics::Window,
}
