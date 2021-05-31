//! Colour mappings for the UI.

// TODO(@MattWindsor91): consider making these configurable.

use crate::{model::split::Pace, presenter::cursor::SplitPosition};
use sdl2::pixels::Color;

/// A set of colours to use in the user interface.
pub struct Set {
    /// Main background colour.
    pub bg: Color,

    // Foreground text for the split editor.
    pub fg_editor: Color,

    // Foreground text for the split editor's current field.
    pub fg_editor_field: Color,

    // Foreground text for headers.
    pub fg_header: Color,

    /// Foreground text for splits already passed.
    pub fg_done: Color,

    /// Foreground text for normal splits.
    pub fg_normal: Color,

    /// Foreground text for the split currently under the cursor.
    pub fg_cursor: Color,

    /// Foreground text for a time when there is no time entered.
    pub fg_time_none: Color,

    /// Foreground text for a time when the run is ahead of comparison.
    pub fg_time_run_ahead: Color,

    /// Foreground text for a time when the split is ahead of comparison.
    /// (Often referred to as a 'gold split'.)
    pub fg_time_split_ahead: Color,

    /// Foreground text for a time when the run is behind comparison.
    pub fg_time_run_behind: Color,
}

/// High-level colour keys.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// Maps to the split editor colour.
    Editor,
    /// Maps to the field editor colour.
    FieldEditor,
    /// Maps to the header colour.
    Header,
    /// Maps to the colour of a split name at a given position.
    Name(SplitPosition),
    /// Maps to a time that hasn't been reported.
    NoTime,
    /// Maps to a pacing colour.
    Pace(Pace),
}

impl Set {
    /// Gets a foreground colour by its key.
    #[must_use]
    pub fn by_key(&self, key: Key) -> Color {
        match key {
            Key::Header => self.fg_header,
            Key::Name(pos) => self.by_split_position(pos),
            Key::NoTime => self.fg_time_none,
            Key::Pace(pace) => self.by_pace(pace),
            Key::Editor => self.fg_editor,
            Key::FieldEditor => self.fg_editor_field,
        }
    }

    #[must_use]
    fn by_pace(&self, pace: Pace) -> Color {
        match pace {
            Pace::PersonalBest => self.fg_time_split_ahead,
            Pace::Behind => self.fg_time_run_ahead,
            Pace::Ahead => self.fg_time_run_behind,
            Pace::Inconclusive => self.fg_normal,
        }
    }

    #[must_use]
    fn by_split_position(&self, sp: SplitPosition) -> Color {
        match sp {
            SplitPosition::Done => self.fg_done,
            SplitPosition::Cursor => self.fg_cursor,
            SplitPosition::Coming => self.fg_normal,
        }
    }
}

/// The default colour set.
pub const SET: Set = Set {
    bg: Color::BLACK,
    fg_editor: Color::MAGENTA,
    fg_editor_field: Color::WHITE,
    fg_cursor: Color::CYAN,
    fg_done: Color::GREY,
    fg_normal: Color::WHITE,
    fg_header: Color::WHITE,
    fg_time_none: Color::GREY,
    fg_time_run_ahead: Color::GREEN,
    fg_time_run_behind: Color::RED,
    fg_time_split_ahead: Color::YELLOW,
};
