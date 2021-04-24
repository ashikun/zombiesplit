//! Colour mappings for the UI.

// TODO(@MattWindsor91): consider making these configurable.

use sdl2::pixels::Color;

/// A set of colours to use in the user interface.
pub struct Set {
    /// Main background colour.
    pub bg: Color,

    /// Foreground text for splits already passed.
    pub fg_done: Color,

    /// Foreground text for normal splits.
    pub fg_normal: Color,

    /// Foreground text for the split currently under the cursor.
    pub fg_cursor: Color,

    /// Foreground text for a time when the run is ahead of comparison.
    pub fg_time_run_ahead: Color,

    /// Foreground text for a time when the split is ahead of comparison.
    /// (Often referred to as a 'gold split'.)
    pub fg_time_split_ahead: Color,

    /// Foreground text for a time when the run is behind comparison.
    pub fg_time_run_behind: Color,
}

impl Set {
    /// The foreground text for split at `pos`, given cursor at `cursor`.
    pub fn fg_split_text(&self, pos: usize, cursor: usize) -> Color {
        use std::cmp::Ordering;
        match pos.cmp(&cursor) {
            Ordering::Less => self.fg_done,
            Ordering::Equal => self.fg_cursor,
            Ordering::Greater => self.fg_normal,
        }
    }
}

/// The default colour set.
pub const SET: Set = Set {
    bg: Color::BLACK,
    fg_cursor: Color::CYAN,
    fg_done: Color::GREY,
    fg_normal: Color::WHITE,
    fg_time_run_ahead: Color::GREEN,
    fg_time_run_behind: Color::RED,
    fg_time_split_ahead: Color::YELLOW,
};
