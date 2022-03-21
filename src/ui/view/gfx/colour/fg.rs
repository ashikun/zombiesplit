//! Foreground colour sets and IDs.

use serde::{Deserialize, Serialize};
use ugly::colour::{self, definition::EGA};

use super::super::super::{
    super::super::model::timing::comparison::pace, presenter::state::cursor::SplitPosition,
};

/// Foreground colour IDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl ugly::colour::id::Fg for Id {}

/// Provides default background colours.
#[must_use]
pub fn defaults() -> colour::Map<Id> {
    let mut result: colour::Map<Id> = [
        (Id::Editor, EGA.bright.cyan),
        (Id::FieldEditor, EGA.bright.white),
        (Id::Header, EGA.bright.red),
        (Id::Normal, EGA.dark.white),
        (Id::Status, EGA.dark.black),
    ]
    .into_iter()
    .collect();
    result.extend(split_pace_defaults());
    result.extend(pace_defaults());
    result.extend(position_defaults());
    result
}

/// Default foreground colours for split pace notes.
fn split_pace_defaults() -> colour::Map<Id> {
    [
        (pace::SplitInRun::Inconclusive, EGA.bright.white),
        (pace::SplitInRun::SplitPersonalBest, EGA.bright.yellow),
        (pace::SplitInRun::BehindAndGaining, EGA.dark.red),
        (pace::SplitInRun::BehindAndLosing, EGA.bright.red),
        (pace::SplitInRun::AheadAndGaining, EGA.bright.green),
        (pace::SplitInRun::AheadAndLosing, EGA.dark.green),
    ]
    .into_iter()
    .map(|(k, v)| (Id::SplitInRunPace(k), v))
    .collect()
}

/// Default foreground colours for pace notes.
fn pace_defaults() -> colour::Map<Id> {
    [
        (pace::Pace::Inconclusive, EGA.bright.white),
        (pace::Pace::PersonalBest, EGA.bright.yellow),
        (pace::Pace::Behind, EGA.bright.red),
        (pace::Pace::Ahead, EGA.bright.green),
    ]
    .into_iter()
    .map(|(k, v)| (Id::Pace(k), v))
    .collect()
}

fn position_defaults() -> colour::Map<Id> {
    [
        (SplitPosition::Coming, EGA.dark.white),
        (SplitPosition::Cursor, EGA.bright.magenta),
        (SplitPosition::Done, EGA.bright.white),
    ]
    .into_iter()
    .map(|(k, v)| (Id::Name(k), v))
    .collect()
}
