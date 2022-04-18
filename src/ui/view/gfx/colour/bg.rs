//! Background colour sets and IDs.

use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};
use ugly::colour::{Definition, Spec};

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

/// Generic background colour map.
///
/// Usually, you'll want either [`UserMap`] or [Map].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct GenMap<T> {
    /// The split editor colour.
    #[serde(default)]
    pub editor: T,
    /// The field editor colour.
    #[serde(default)]
    pub field_editor: T,
    /// The status bar background colour.
    #[serde(default)]
    pub status: T,
    /// The main window colour.
    #[serde(default)]
    pub window: T,
}

/// User-provided, partial palette.
pub type UserMap = GenMap<Option<Spec>>;

/// Compiled background colour map.
pub type Map = GenMap<Definition>;

impl Map {
    /// Adds the user colour overrides in `user` into this palette.
    pub fn add_user(&mut self, user: UserMap) {
        for i in [Id::Editor, Id::FieldEditor, Id::Status, Id::Window] {
            if let Some(v) = user[i] {
                self[i] = v.into_definition();
            }
        }
    }
}

impl<T> Index<Id> for GenMap<T> {
    type Output = T;

    fn index(&self, index: Id) -> &T {
        match index {
            Id::Window => &self.window,
            Id::Editor => &self.editor,
            Id::FieldEditor => &self.field_editor,
            Id::Status => &self.status,
        }
    }
}

/// We can index using an optional ID; `None` is hardwired to transparency.
impl Index<Option<Id>> for Map {
    type Output = Definition;

    fn index(&self, index: Option<Id>) -> &Self::Output {
        index.map_or(&ugly::colour::definition::TRANSPARENT, |i| self.index(i))
    }
}

/// We can mutably index, but only using non-optional IDs.
impl IndexMut<Id> for Map {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        match index {
            Id::Window => &mut self.window,
            Id::Editor => &mut self.editor,
            Id::FieldEditor => &mut self.field_editor,
            Id::Status => &mut self.status,
        }
    }
}

impl ugly::resource::Map<Definition> for Map {
    type Id = Option<Id>;

    fn get(&self, k: Self::Id) -> &Definition {
        self.index(k)
    }
}
