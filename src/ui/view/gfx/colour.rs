//! Colour mappings for the UI.

pub mod bg;
pub mod definition;
pub mod error;
pub mod fg;

use serde::{Deserialize, Serialize};

use crate::ui::view::gfx::colour::fg::Id;
pub use error::{Error, Result};

/// Set of colour mappings used in the UI.
#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Set {
    /// Foreground colours.
    pub fg: fg::Set,
    /// Background colours.
    pub bg: bg::Set,
}

/// Pair of foreground and optional background identifiers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pair {
    /// The foreground colour.
    pub fg: fg::Id,
    /// The optional background colour.
    pub bg: Option<bg::Id>,
}

/// Lifts a foreground colour into a pair with no background colour.
impl From<fg::Id> for Pair {
    fn from(fg: Id) -> Self {
        Pair { fg, bg: None }
    }
}
