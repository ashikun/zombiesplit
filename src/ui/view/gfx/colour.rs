//! Colour palettes for the UI, used for `ugly`.
//!
//! Palettes come in the form of RON (rusty object notation) files.  This is primarily because
//! palette indices are fairly complex enums, and it's easier to handle them using RON than it is
//! in TOML.

use thiserror::Error;

pub mod bg;
pub mod default;
pub mod fg;

/// Palette of colour mappings used in the UI.
pub type Palette = ugly::colour::Palette<fg::Map, bg::Map>;

/// Palette of user overrides.
///
/// Unlike [Palette], this can be partial.
pub type UserPalette = ugly::colour::Palette<fg::UserMap, bg::UserMap>;

/// Loads a palette from a TOML file.
///
/// # Errors
///
/// Fails if we can't find the file, or if it isn't valid TOML.
pub fn load_toml(path: impl AsRef<std::path::Path>) -> Result<UserPalette> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

/// Merges a user palette into a palette.
pub fn add_user(palette: &mut Palette, user: &UserPalette) {
    palette.fg.add_user(user.fg);
    palette.bg.add_user(user.bg);
}

/// Enumeration of palette loading errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Couldn't load palette file.
    #[error("couldn't load palette file")]
    Io(#[from] std::io::Error),
    /// Couldn't deserialise palette from RON.
    #[error("couldn't deserialise palette from TOML")]
    Toml(#[from] toml::de::Error),
}

/// Shorthand for a result over colour palette loading.
pub type Result<T> = std::result::Result<T, Error>;

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
