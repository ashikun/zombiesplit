//! Foreground colour sets and IDs.

use serde::{Deserialize, Serialize};

use super::{
    super::super::presenter::state::cursor::SplitPosition,
    definition::{Colour, EGA},
};
use crate::model::timing::comparison::pace;

/// Foreground colour IDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Id {
    /// The split editor colour.
    Editor,
    /// The field editor colour.
    FieldEditor,
    /// The header colour.
    Header,
    /// The colour of a split name at a given position.
    Name(SplitPosition),
    /// Neutral colour.
    Normal,
    /// A split-in-run pacing colour.
    SplitInRunPace(pace::SplitInRun),
    /// A pacing colour.
    Pace(pace::Pace),
    /// Foreground text for the status bar.
    Status,
}

/// There is a default foreground colour, `Normal`.
impl Default for Id {
    fn default() -> Self {
        Self::Normal
    }
}

/// A set of foreground colours.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Set {
    // Foreground text for the split editor.
    pub editor: Colour,

    // Foreground text for the split editor's current field.
    pub editor_field: Colour,

    // Foreground text for headers.
    pub header: Colour,

    /// Foreground text for splits already passed.
    pub done: Colour,

    /// Normal foreground text.
    pub normal: Colour,

    /// Foreground text for the split currently under the cursor.
    pub cursor: Colour,

    /// Foreground text for times with an associated pace.
    pub pace: Pace,

    /// Foreground text for the status bar.
    pub status: Colour,
}

impl Set {
    /// Gets a foreground colour by its ID.
    #[must_use]
    pub fn get(&self, id: Id) -> Colour {
        match id {
            Id::Header => self.header,
            Id::Name(pos) => self.by_split_position(pos),
            Id::Normal => self.normal,
            Id::SplitInRunPace(pace) => self.pace[pace],
            Id::Pace(pace) => self.pace[pace],
            Id::Editor => self.editor,
            Id::FieldEditor => self.editor_field,
            Id::Status => self.status,
        }
    }

    #[must_use]
    fn by_split_position(&self, sp: SplitPosition) -> Colour {
        match sp {
            SplitPosition::Done => self.done,
            SplitPosition::Cursor => self.cursor,
            SplitPosition::Coming => self.normal,
        }
    }
}

/// Provides default foreground colours.
impl Default for Set {
    fn default() -> Self {
        Self {
            editor: EGA.bright.cyan,
            editor_field: EGA.bright.white,
            header: EGA.bright.red,
            done: EGA.bright.black,
            normal: EGA.dark.white,
            cursor: EGA.bright.magenta,
            status: EGA.dark.black,
            pace: Pace::default(),
        }
    }
}

/// Set of foreground colours for pace notes.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Pace {
    /// Colour used for inconclusive times.
    pub inconclusive: Colour,
    /// Colour used for behind-of-pace times.
    pub behind: Colour,
    /// Colour used for behind-and-gaining times.
    pub behind_gaining: Colour,
    /// Colour used for ahead-but-losing times.
    pub ahead_losing: Colour,
    /// Colour used for ahead-of-pace times.
    pub ahead: Colour,
    /// Colour used for personal-best times.
    pub personal_best: Colour,
}

/// Provides default pace foreground colours.
impl Default for Pace {
    fn default() -> Self {
        Self {
            inconclusive: EGA.bright.white,
            ahead: EGA.bright.green,
            ahead_losing: EGA.dark.green,
            behind_gaining: EGA.dark.red,
            behind: EGA.bright.red,
            personal_best: EGA.bright.yellow,
        }
    }
}

/// We can index into a pace set by split-in-run pace note, to get a colour.
impl std::ops::Index<pace::SplitInRun> for Pace {
    type Output = Colour;

    fn index(&self, index: pace::SplitInRun) -> &Self::Output {
        match index {
            pace::SplitInRun::SplitPersonalBest => &self.personal_best,
            pace::SplitInRun::Inconclusive => &self.inconclusive,
            pace::SplitInRun::BehindAndGaining => &self.behind_gaining,
            pace::SplitInRun::BehindAndLosing => &self.behind,
            pace::SplitInRun::AheadAndGaining => &self.ahead,
            pace::SplitInRun::AheadAndLosing => &self.ahead_losing,
        }
    }
}

/// We can index into a pace set by pace note, to get a colour.
impl std::ops::Index<pace::Pace> for Pace {
    type Output = Colour;

    fn index(&self, index: pace::Pace) -> &Self::Output {
        match index {
            pace::Pace::PersonalBest => &self.personal_best,
            pace::Pace::Behind => &self.behind,
            pace::Pace::Ahead => &self.ahead,
            pace::Pace::Inconclusive => &self.inconclusive,
        }
    }
}
