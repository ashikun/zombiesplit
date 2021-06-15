/*!
Models for game configuration.

These models represent the way in which zombiesplit is fed new games to track
(not to be confused with the models that represent in-database game data).
*/

use super::super::time;

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::Read,
    path::Path,
    str::FromStr,
};
use thiserror::Error;

/// Configuration for a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The name of the game.
    pub name: String,
    /// Map of split segments for the game.
    pub segments: ShortNameMap<Segment>,
    /// Map of split configurations for the game.
    pub splits: ShortNameMap<Split>,
    /// Map of categories for the game.
    pub categories: ShortNameMap<Category>,
}

impl Config {
    /// Loads a game config from TOML.
    ///
    /// # Errors
    ///
    /// Returns an error if `path` does not exist, is not readable, or does
    /// not contain valid TOML.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}

/// A run category.
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    /// The display name of the category.
    pub name: String,
    /// The list of segments that make up the category.
    #[serde(default)]
    pub segments: Vec<ShortName>,
}

/// A configured split segment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    /// The name of the segment.
    pub name: String,
    #[serde(default)]
    /// The splits inhabiting the segment.
    pub splits: Vec<ShortName>,
}

/// A configured split.
#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    /// The split name.
    pub name: String,
    /// The set of records configured for this split.
    #[serde(default)]
    pub records: ShortNameMap<Record>,
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

/// A short name, used to look up items in the game database.
pub type ShortName = String;
/// A map from short names to some type.
pub type ShortNameMap<T> = HashMap<ShortName, T>;

/// Enumeration of errors occurring when interpreting game config.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error reading game config")]
    Io(#[from] std::io::Error),
    #[error("Error parsing game config from TOML")]
    Toml(#[from] toml::de::Error),
}

/// Shorthand for results over [Error].
type Result<T> = std::result::Result<T, Error>;
