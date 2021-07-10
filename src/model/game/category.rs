//! Models relating to the set of categories attached to a game.
use std::{fmt::Display, str::FromStr};

use super::super::short;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A reference to the category of a game using a pair of short names.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ShortDescriptor {
    /// The shortname of the game.
    pub game: short::Name,
    /// The shortname of the category.
    pub category: short::Name,
}

impl ShortDescriptor {
    /// Constructs a new short descriptor with the given game and category.
    #[must_use]
    pub fn new(game: &str, category: &str) -> Self {
        Self {
            game: game.to_owned(),
            category: category.to_owned(),
        }
    }
}

impl Display for ShortDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.game, self.category)
    }
}

impl FromStr for ShortDescriptor {
    type Err = ShortDescriptorError;

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

/// Full, displayable metadata about a category of a game.
pub struct Info {
    /// The name of the game.
    pub game: String,
    /// The name of the category.
    pub category: String,
    /// The short descriptor of the category and game.
    pub short: ShortDescriptor,
}

/// Information about the number of attempts a game-category has had.
pub struct AttemptInfo {
    /// The number of runs stored in total.
    pub total: usize,
    /// The number of runs stored and marked as completed.
    pub completed: usize,
}
