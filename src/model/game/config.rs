/*!
Models for game configuration.

These models represent the way in which zombiesplit is fed new games to track
(not to be confused with the models that represent in-database game data).
*/

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::model::timing::time;

use super::super::short;

/// Configuration for a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The name of the game.
    pub name: String,
    /// Map of split segments for the game.
    pub segments: short::Map<Segment>,
    /// Map of split configurations for the game.
    pub splits: short::Map<Split>,
    /// Map of categories for the game.
    pub categories: short::Map<Category>,
}

/// A run category.
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    /// The display name of the category.
    pub name: String,
    /// The list of segments that make up the category.
    #[serde(default)]
    pub segments: Vec<short::Name>,
}

/// A configured split segment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    /// The name of the segment.
    pub name: String,
    #[serde(default)]
    /// The splits inhabiting the segment.
    pub splits: Vec<short::Name>,
}

/// A configured split.
#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    /// The split name.
    pub name: String,
    /// The set of records configured for this split.
    #[serde(default)]
    pub records: short::Map<Record>,
}

/// A configured record.
#[derive(SerializeDisplay, DeserializeFromStr, Debug)]
pub struct Record {
    /// The time.
    pub time: time::Time,
}

impl Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time)
    }
}

impl FromStr for Record {
    type Err = time::Error; // for now

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Record { time: s.parse()? })
    }
}
