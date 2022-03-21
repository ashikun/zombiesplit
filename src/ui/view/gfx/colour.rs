//! Colour palettes for the UI, used for `ugly`.
//!
//! Palettes come in the form of RON (rusty object notation) files.  This is primarily because
//! palette indices are fairly complex enums, and it's easier to handle them using RON than it is
//! in TOML.

use thiserror::Error;

pub mod bg;
pub mod fg;

/// Palette of colour mappings used in the UI.
pub type Palette = ugly::colour::definition::MapSet<fg::Id, bg::Id>;

/// Loads a palette from a RON file.
///
/// # Errors
///
/// Fails if we can't find the file, or if it isn't valid RON.
pub fn load_ron(path: impl AsRef<std::path::Path>) -> Result<Palette> {
    Ok(ron::from_str(&std::fs::read_to_string(path)?)?)
}

/// Enumeration of palette loading errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Couldn't load palette file.
    #[error("couldn't load palette file")]
    Io(#[from] std::io::Error),
    /// Couldn't deserialise palette from RON.
    #[error("couldn't deserialise palette from RON")]
    Ron(#[from] ron::Error),
}

/// Shorthand for a result over colour palette loading.
pub type Result<T> = std::result::Result<T, Error>;

/// Gets the default colour set.
#[must_use]
pub fn defaults() -> Palette {
    Palette {
        fg: fg::defaults(),
        bg: bg::defaults(),
    }
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
    fn from(fg: fg::Id) -> Self {
        Pair { fg, bg: None }
    }
}
