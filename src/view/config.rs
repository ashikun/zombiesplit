//! View configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::gfx::{font, metrics};

/// Top-level UI configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Font configuration.
    pub fonts: HashMap<font::Id, font::Config>,
    /// Window metrics.
    pub window: metrics::Window,
}
