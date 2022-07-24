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
use thiserror::Error;

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

impl Config {
    /// Wrapper for `categories.get`.
    ///
    /// # Errors
    ///
    /// Fails if the category at `short` is not present.
    pub fn category(&self, short: impl Into<short::Name>) -> Result<&Category> {
        let short = short.into();
        self.categories
            .get(&short)
            .ok_or(Error::MissingCategory { short })
    }
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

impl Category {
    /// Iterates over the full split group information for this category.
    ///
    /// This iterator requires information from the underlying `game`, and returns results that may
    /// fail if the `game` is missing the referenced data.
    pub fn full_segments<'cfg>(
        &'cfg self,
        game: &'cfg Config,
    ) -> impl Iterator<Item = Result<(short::Name, &'cfg Segment)>> + 'cfg {
        self.segments.iter().map(|sn| {
            game.segments
                .get_key_value(sn)
                .map(|(k, v)| (*k, v))
                .ok_or_else(|| Error::MissingSegment {
                    short: *sn,
                    in_category: self.name.clone(),
                })
        })
    }
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

impl Segment {
    /// Iterates over the full split information for this segment.
    ///
    /// This iterator requires information from the underlying `game`, and returns results that may
    /// fail if the `game` is missing the referenced split.
    pub fn full_splits<'cfg>(
        &'cfg self,
        game: &'cfg Config,
    ) -> impl Iterator<Item = Result<(short::Name, &'cfg Split)>> + 'cfg {
        self.splits.iter().map(|sn| {
            game.splits
                .get_key_value(sn)
                .map(|(k, v)| (*k, v))
                .ok_or_else(|| Error::MissingSplit {
                    short: *sn,
                    in_segment: self.name.clone(),
                })
        })
    }
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

/// Errors when resolving parts of game configuration.
///
/// These errors work equally well when referring to resolving aspects of a static game
/// configuration, or when trying to resolve their equivalents inside a game database.
#[derive(Debug, Error)]
pub enum Error {
    // TODO: deduplicate this with the database errors?
    /// A requested category was missing.
    #[error("couldn't find category {short}")]
    MissingCategory { short: short::Name },

    /// A category referenced a segment that is missing in the configuration or database.
    #[error("couldn't find segment {short} requested by category {in_category}")]
    MissingSegment {
        short: short::Name,
        in_category: String,
    },

    /// A segment referenced a split not inserted in the configuration or database.
    #[error("couldn't find split {short} requested by segment {in_segment}")]
    MissingSplit {
        short: short::Name,
        in_segment: String,
    },
}

/// Shorthand for results over expanding game config.
pub type Result<T> = std::result::Result<T, Error>;
