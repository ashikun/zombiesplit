//! Colour mappings for the UI.

pub mod bg;
pub mod definition;
pub mod error;
pub mod fg;

use serde::{Deserialize, Serialize};

pub use error::{Error, Result};

/// Set of colour mappings used in the UI.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Set {
    /// Foreground colours.
    pub fg: fg::Set,
    /// Background colours.
    pub bg: bg::Set,
}
