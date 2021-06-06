//! Foreground colour sets and IDs.

use super::definition::Colour;
use crate::{model::pace::Pace, presenter::cursor::SplitPosition};
use serde::{Deserialize, Serialize};

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
    /// A time that hasn't been reported.
    NoTime,
    /// A pacing colour.
    Pace(Pace),
}

/// A set of foreground colours.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Set {
    // Foreground text for the split editor.
    pub editor: Colour,

    // Foreground text for the split editor's current field.
    pub editor_field: Colour,

    // Foreground text for headers.
    pub header: Colour,

    /// Foreground text for splits already passed.
    pub done: Colour,

    /// Foreground text for normal splits.
    pub normal: Colour,

    /// Foreground text for the split currently under the cursor.
    pub cursor: Colour,

    /// Foreground text for a time when there is no time entered.
    pub time_none: Colour,

    /// Foreground text for a time when the run is ahead of comparison.
    pub time_run_ahead: Colour,

    /// Foreground text for a time when the split is ahead of comparison.
    /// (Often referred to as a 'gold split'.)
    pub time_split_ahead: Colour,

    /// Foreground text for a time when the run is behind comparison.
    pub time_run_behind: Colour,
}

impl Set {
    /// Gets a foreground colour by its ID.
    #[must_use]
    pub fn get(&self, id: Id) -> Colour {
        match id {
            Id::Header => self.header,
            Id::Name(pos) => self.by_split_position(pos),
            Id::NoTime => self.time_none,
            Id::Pace(pace) => self.by_pace(pace),
            Id::Editor => self.editor,
            Id::FieldEditor => self.editor_field,
        }
    }

    #[must_use]
    fn by_pace(&self, pace: Pace) -> Colour {
        match pace {
            Pace::PersonalBest => self.time_split_ahead,
            Pace::Behind => self.time_run_ahead,
            Pace::Ahead => self.time_run_behind,
            Pace::Inconclusive => self.normal,
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
