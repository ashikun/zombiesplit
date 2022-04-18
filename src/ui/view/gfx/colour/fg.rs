//! Foreground colour sets and IDs.
//!
//! For the default palette, see `super::default`.

use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use ugly::colour::{Definition, Spec};

use super::super::super::{
    super::super::model::timing::comparison::pace, presenter::state::cursor::SplitPosition,
};

/// Foreground colour IDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Id {
    /// Neutral colour.
    Normal,
    /// The split editor colour.
    Editor,
    /// The field editor colour.
    FieldEditor,
    /// The header colour.
    Header,
    /// Foreground text for the status bar.
    Status,
    /// The colour of a split name at a given position.
    Name(SplitPosition),
    /// A split-in-run pacing colour.
    SplitInRunPace(pace::SplitInRun),
    /// A pacing colour.
    Pace(pace::Pace),
}

/// There is a default foreground colour, `Normal`.
impl Default for Id {
    fn default() -> Self {
        Self::Normal
    }
}

/// Generic background colour map.
///
/// Usually, you'll want either [`UserMap`] or [Map].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct GenMap<T> {
    /// Neutral colour.
    #[serde(default)]
    pub normal: T,
    /// The split editor colour.
    #[serde(default)]
    pub editor: T,
    /// The field editor colour.
    #[serde(default)]
    pub field_editor: T,
    /// The header colour.
    #[serde(default)]
    pub header: T,
    /// Foreground text for the status bar.
    #[serde(default)]
    pub status: T,
    /// The colour of a split name at a given position.
    #[serde(default)]
    pub name: PositionMap<T>,
    /// Pace colours.
    #[serde(default)]
    pub pace: PaceMap<T>,
}

/// Map of user-supplied colour definitions.
pub type UserMap = GenMap<Option<Spec>>;

/// Compiled background colour map.
pub type Map = GenMap<Definition>;

impl Map {
    /// Adds the user colour overrides in `user` into this palette.
    pub fn add_user(&mut self, user: UserMap) {
        for i in [
            Id::Normal,
            Id::Editor,
            Id::FieldEditor,
            Id::Header,
            Id::Status,
        ] {
            if let Some(v) = user[i] {
                self[i] = v.into_definition();
            }
        }
        self.name.add_user(user.name);
        self.pace.add_user(user.pace);
    }
}

impl<T> Index<Id> for GenMap<T> {
    type Output = T;

    fn index(&self, index: Id) -> &Self::Output {
        match index {
            Id::Normal => &self.normal,
            Id::Editor => &self.editor,
            Id::FieldEditor => &self.field_editor,
            Id::Header => &self.header,
            Id::Status => &self.status,
            Id::Name(n) => self.name.index(n),
            Id::SplitInRunPace(p) => self.pace.get_split_in_run(p),
            Id::Pace(p) => self.pace.get(p),
        }
    }
}

impl IndexMut<Id> for Map {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        match index {
            Id::Normal => &mut self.normal,
            Id::Editor => &mut self.editor,
            Id::FieldEditor => &mut self.field_editor,
            Id::Header => &mut self.header,
            Id::Status => &mut self.status,
            Id::Name(n) => self.name.index_mut(n),
            Id::SplitInRunPace(p) => self.pace.get_split_in_run_mut(p),
            Id::Pace(p) => self.pace.get_mut(p),
        }
    }
}

impl ugly::resource::Map<Definition> for Map {
    type Id = Id;

    fn get(&self, k: Self::Id) -> &Definition {
        self.index(k)
    }
}

/// Map of pace colours.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PaceMap<T> {
    /// Inconclusive pace.
    #[serde(default)]
    pub inconclusive: T,
    /// Behind.
    #[serde(default)]
    pub behind: T,
    /// Behind, but gaining time.
    #[serde(default)]
    pub behind_but_gaining: T,
    /// Ahead, but losing time.
    #[serde(default)]
    pub ahead_but_losing: T,
    /// Ahead.
    #[serde(default)]
    pub ahead: T,
    /// Personal best.
    #[serde(default)]
    pub personal_best: T,
}

impl PaceMap<Definition> {
    fn add_user(&mut self, user: PaceMap<Option<Spec>>) {
        for i in [
            pace::SplitInRun::Inconclusive,
            pace::SplitInRun::SplitPersonalBest,
            pace::SplitInRun::BehindAndLosing,
            pace::SplitInRun::BehindAndGaining,
            pace::SplitInRun::AheadAndLosing,
            pace::SplitInRun::AheadAndGaining,
        ] {
            if let Some(v) = user.get_split_in_run(i) {
                *self.get_split_in_run_mut(i) = v.into_definition();
            }
        }
    }
}

impl<T> PaceMap<T> {
    fn get_split_in_run(&self, p: pace::SplitInRun) -> &T {
        match p {
            pace::SplitInRun::Inconclusive => &self.inconclusive,
            pace::SplitInRun::SplitPersonalBest => &self.personal_best,
            pace::SplitInRun::BehindAndLosing => &self.behind,
            pace::SplitInRun::BehindAndGaining => &self.behind_but_gaining,
            pace::SplitInRun::AheadAndLosing => &self.ahead_but_losing,
            pace::SplitInRun::AheadAndGaining => &self.ahead,
        }
    }

    fn get(&self, p: pace::Pace) -> &T {
        match p {
            pace::Pace::Inconclusive => &self.inconclusive,
            pace::Pace::PersonalBest => &self.personal_best,
            pace::Pace::Behind => &self.behind,
            pace::Pace::Ahead => &self.ahead,
        }
    }

    fn get_split_in_run_mut(&mut self, p: pace::SplitInRun) -> &mut T {
        match p {
            pace::SplitInRun::Inconclusive => &mut self.inconclusive,
            pace::SplitInRun::SplitPersonalBest => &mut self.personal_best,
            pace::SplitInRun::BehindAndLosing => &mut self.behind,
            pace::SplitInRun::BehindAndGaining => &mut self.behind_but_gaining,
            pace::SplitInRun::AheadAndLosing => &mut self.ahead_but_losing,
            pace::SplitInRun::AheadAndGaining => &mut self.ahead,
        }
    }

    fn get_mut(&mut self, p: pace::Pace) -> &mut T {
        match p {
            pace::Pace::Inconclusive => &mut self.inconclusive,
            pace::Pace::PersonalBest => &mut self.personal_best,
            pace::Pace::Behind => &mut self.behind,
            pace::Pace::Ahead => &mut self.ahead,
        }
    }
}

/// Position colour map.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PositionMap<T> {
    /// Colour for splits after the cursor.
    #[serde(default)]
    pub coming: T,
    /// Colour for splits on the cursor.
    #[serde(default)]
    pub cursor: T,
    /// Colour for splits before the cursor.
    #[serde(default)]
    pub done: T,
}

impl<T> Index<SplitPosition> for PositionMap<T> {
    type Output = T;

    fn index(&self, k: SplitPosition) -> &Self::Output {
        match k {
            SplitPosition::Coming => &self.coming,
            SplitPosition::Cursor => &self.cursor,
            SplitPosition::Done => &self.done,
        }
    }
}

impl<T> IndexMut<SplitPosition> for PositionMap<T> {
    fn index_mut(&mut self, k: SplitPosition) -> &mut T {
        match k {
            SplitPosition::Coming => &mut self.coming,
            SplitPosition::Cursor => &mut self.cursor,
            SplitPosition::Done => &mut self.done,
        }
    }
}

impl PositionMap<Definition> {
    fn add_user(&mut self, user: PositionMap<Option<Spec>>) {
        for i in [
            SplitPosition::Coming,
            SplitPosition::Cursor,
            SplitPosition::Done,
        ] {
            if let Some(v) = user[i] {
                self[i] = v.into_definition();
            }
        }
    }
}
