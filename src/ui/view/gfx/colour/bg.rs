//! Background colour sets and IDs.

use super::definition::{Colour, EGA};
use serde::{Deserialize, Serialize};

/// Background colour IDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Id {
    /// The main window colour.
    Window,
    /// The split editor colour.
    Editor,
    /// The field editor colour.
    FieldEditor,
}

/// A set of background colours.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Set {
    /// Main background colour.
    pub window: Colour,
    /// Background for the split editor.
    pub editor: Colour,
    /// Background for the field editor.
    pub editor_field: Colour,
}

/// Provides default background colours.
impl Default for Set {
    fn default() -> Self {
        Self {
            window: EGA.dark.black,
            editor: EGA.dark.blue,
            editor_field: EGA.dark.cyan,
        }
    }
}

impl Set {
    /// Gets a background colour by its ID.
    #[must_use]
    pub fn get(&self, id: Id) -> Colour {
        match id {
            Id::Window => self.window,
            Id::Editor => self.editor,
            Id::FieldEditor => self.editor_field,
        }
    }
}
