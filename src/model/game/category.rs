//! Models relating to the set of categories attached to a game.
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use super::super::short;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A reference to the category of a game using a pair of short names.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ShortDescriptor {
    /// The shortname of the game.
    pub game: short::Name,
    /// The shortname of the category.
    pub category: short::Name,
}

impl ShortDescriptor {
    /// Constructs a new short descriptor with the given game and category.
    #[must_use]
    pub fn new(game: impl Into<short::Name>, category: impl Into<short::Name>) -> Self {
        // TODO(@MattWindsor91): deal with slashes; parsing/displaying assumes there aren't any.
        Self {
            game: game.into(),
            category: category.into(),
        }
    }
}

impl Display for ShortDescriptor {
    /// Formats a short descriptor using slashes.
    ///
    /// ```
    /// use zombiesplit::model::game::category::ShortDescriptor;
    ///
    /// assert_eq!("scd11/btg-sonic", format!("{}", ShortDescriptor::new("scd11", "btg-sonic")));
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.game, self.category)
    }
}

impl FromStr for ShortDescriptor {
    type Err = ShortDescriptorError;

    /// Parses a short descriptor from a slash delimited string.
    ///
    /// ```
    /// use zombiesplit::model::game::category::ShortDescriptor;
    ///
    /// let got: ShortDescriptor = "scd11/btg-sonic".parse().expect("unexpected parse error");
    /// assert_eq!(ShortDescriptor::new("scd11", "btg-sonic"), got);
    ///
    /// "scd11".parse::<ShortDescriptor>().expect_err("can't parse without one slash");
    /// "scd11/btg/sonic".parse::<ShortDescriptor>().expect_err("can't parse with three slashes");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split('/').collect();
        if splits.len() == 2 {
            Ok(Self::new(splits[0], splits[1]))
        } else {
            Err(ShortDescriptorError::WrongSlashCount)
        }
    }
}

/// Parsing errors for short descriptors.
#[derive(Debug, Error)]
pub enum ShortDescriptorError {
    /// Incorrect number of slashes in a short descriptor.
    #[error("short descriptors need exactly one slash")]
    WrongSlashCount,
}

/// Full, displayable metadata about a target (game/category pair).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target {
    /// The name of the game.
    pub game: String,
    /// The name of the category.
    pub category: String,
    /// The short descriptor of the category and game.
    pub short: ShortDescriptor,
}

/// Information about the number of attempts a game-category has had.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptInfo {
    /// The number of runs stored in total.
    ///
    /// Note that this is _one below_ the number of the current run.
    pub total: usize,
    /// The number of runs stored and marked as completed.
    pub completed: usize,
}

impl AttemptInfo {
    /// Increments the attempt counter(s).
    ///
    /// If `is_completed` is true, we increment the completed counter too.
    ///
    /// ```
    /// use zombiesplit::model::game::category::AttemptInfo;
    /// let mut info = AttemptInfo::default();
    /// info.increment(false);
    /// info.increment(true);
    /// assert_eq!(2, info.total);
    /// assert_eq!(1, info.completed);
    /// ```
    pub fn increment(&mut self, is_completed: bool) {
        self.total += 1;
        if is_completed {
            self.completed += 1;
        }
    }
}

/// Display attempt info as 'total (completed)'.
impl Display for AttemptInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.total, self.completed)
    }
}
