//! Colour mappings for the UI.

use crate::{model::pace::Pace, presenter::cursor::SplitPosition};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// A colour.
#[derive(Copy, Clone, Debug, DeserializeFromStr, SerializeDisplay)]
pub struct Colour(css_color_parser::Color);

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Colour {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Colour(s.parse()?))
    }
}

impl From<Colour> for sdl2::pixels::Color {
    fn from(c: Colour) -> Self {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let a = (255.0 * c.0.a).round() as u8;
        Self::RGBA(c.0.r, c.0.g, c.0.b, a)
    }
}

/// Errors that can occur when parsing a colour.
#[derive(Debug, Error)]
pub enum Error {
    #[error("malformed colour")]
    Malformed(#[from] css_color_parser::ColorParseError),
}

/// Shorthand for result type.
pub type Result<T> = std::result::Result<T, Error>;

/// A set of foreground colours.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ForegroundSet {
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

/// A set of background colours.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundSet {
    /// Main background colour.
    pub window: Colour,
}

/// A set of colours to use in the user interface.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Set {
    /// Foreground colours.
    pub fg: ForegroundSet,
    /// Background colours.
    pub bg: BackgroundSet
}

/// Foreground colour keys.
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

impl ForegroundSet {
    /// Gets a foreground colour by its key.
    #[must_use]
    pub fn by_key(&self, key: Key) -> Colour {
        match key {
            Key::Header => self.header,
            Key::Name(pos) => self.by_split_position(pos),
            Key::NoTime => self.time_none,
            Key::Pace(pace) => self.by_pace(pace),
            Key::Editor => self.editor,
            Key::FieldEditor => self.editor_field,
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
