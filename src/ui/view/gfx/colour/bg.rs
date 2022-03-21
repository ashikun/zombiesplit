//! Background colour sets and IDs.

use serde::{Deserialize, Serialize};
use ugly::colour;

/// Background colour IDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Id {
    /// The main window colour.
    Window,
    /// The split editor colour.
    Editor,
    /// The field editor colour.
    FieldEditor,
    /// The status bar background colour.
    Status,
}

impl ugly::colour::id::Bg for Id {}

/// Provides default background colours.
#[must_use]
pub fn defaults() -> colour::Map<Id> {
    [
        (Id::Window, colour::definition::EGA.dark.black),
        (Id::Editor, colour::definition::EGA.dark.blue),
        (Id::FieldEditor, colour::definition::EGA.dark.cyan),
        (Id::Status, colour::definition::EGA.dark.white),
    ]
    .into_iter()
    .collect()
}
