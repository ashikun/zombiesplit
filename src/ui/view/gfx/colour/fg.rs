//! Foreground colour sets and IDs.

use super::{super::super::presenter::cursor::SplitPosition, definition::Colour};
use crate::model::comparison::{pace, Pace};
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
    /// A split-in-run pacing colour.
    SplitInRunPace(pace::SplitInRun),
    /// A pacing colour.
    Pace(Pace),
}

/// There is a default foreground colour, but it is fairly arbitrary at the moment.
impl Default for Id {
    fn default() -> Self {
        Self::Name(SplitPosition::Coming)
    }
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

    /// Foreground text for a time when the run is ahead of comparison,
    /// and the split was also ahead.
    pub time_ahead: Colour,

    /// Foreground text for a time when the run is ahead of comparison,
    /// but the split was behind.
    pub time_ahead_losing: Colour,

    /// Foreground text for a time when the run is behind comparison,
    /// but the split was also ahead.
    pub time_behind_gaining: Colour,

    /// Foreground text for a time when the run is behind comparison.
    pub time_behind: Colour,

    /// Foreground text for a time when the split is ahead of comparison.
    /// (Often referred to as a 'gold split'.)
    pub time_split_ahead: Colour,
}

impl Set {
    /// Gets a foreground colour by its ID.
    #[must_use]
    pub fn get(&self, id: Id) -> Colour {
        match id {
            Id::Header => self.header,
            Id::Name(pos) => self.by_split_position(pos),
            Id::NoTime => self.time_none,
            Id::SplitInRunPace(pace) => self.by_split_in_run_pace(pace),
            Id::Pace(pace) => self.by_pace(pace),
            Id::Editor => self.editor,
            Id::FieldEditor => self.editor_field,
        }
    }

    #[must_use]
    fn by_split_in_run_pace(&self, pace: pace::SplitInRun) -> Colour {
        match pace {
            pace::SplitInRun::SplitPersonalBest => self.time_split_ahead,
            pace::SplitInRun::Inconclusive => self.normal,
            pace::SplitInRun::BehindAndGaining => self.time_behind_gaining,
            pace::SplitInRun::BehindAndLosing => self.time_behind,
            pace::SplitInRun::AheadAndGaining => self.time_ahead,
            pace::SplitInRun::AheadAndLosing => self.time_ahead_losing,
        }
    }

    #[must_use]
    fn by_pace(&self, pace: Pace) -> Colour {
        match pace {
            Pace::PersonalBest => self.time_split_ahead,
            Pace::Behind => self.time_behind,
            Pace::Ahead => self.time_ahead,
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

/// Provides default foreground colours.
impl Default for Set {
    fn default() -> Self {
        Self {
            editor: Colour::rgb(0x55, 0xFF, 0xFF),       // EGA bright teal
            editor_field: Colour::rgb(0xFF, 0xFF, 0xFF), // EGA bright white
            header: Colour::rgb(0xFF, 0x55, 0x55),       // EGA bright red
            done: Colour::rgb(0x55, 0x55, 0x55),         // EGA bright black
            normal: Colour::rgb(0xAA, 0xAA, 0xAA),       // EGA white
            cursor: Colour::rgb(0xFF, 0x55, 0xFF),       // EGA bright magenta
            time_none: Colour::rgb(0xAA, 0xAA, 0xAA),    // EGA white
            time_ahead: Colour::rgb(0x55, 0xFF, 0xFF),   // EGA bright green
            time_ahead_losing: Colour::rgb(0x00, 0xAA, 0xAA), // EGA green
            time_behind_gaining: Colour::rgb(0xAA, 0x00, 0x00), // EGA red
            time_behind: Colour::rgb(0xFF, 0xAA, 0xAA),  // EGA bright red
            time_split_ahead: Colour::rgb(0xFF, 0xFF, 0x55), // EGA bright yellow
        }
    }
}
