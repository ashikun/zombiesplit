//! Configuration structs for games, split groups, splits, records, and categories.
use crate::model::{
    run::{self, Metadata},
    split, time,
};

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
pub struct Game {
    /// The name of the game.
    pub name: String,
    /// Map of split groups for the game.
    pub groups: HashMap<GroupId, Group>,
    /// Map of categories for the game.
    pub categories: HashMap<CategoryId, Category>,
}

impl Game {
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

    /// Creates a new run using the game as a template.
    ///
    /// This is a temporary function that will likely go away once we implement
    /// sqlite integration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration references a category or group
    /// that is not available elsewhere in the configuration.
    pub fn to_run(&self, category: &str) -> Result<run::Run> {
        let cat = self.expand_category(category)?;
        // TODO(@MattWindsor91): check groups are valid

        Ok(run::Run {
            attempt: 0,
            metadata: self.to_metadata(&cat),
            splits: self.to_splits(&cat)?,
            // TODO(@MattWindsor91): add comparisons
            comparisons: vec![],
        })
    }

    fn expand_category(&self, category: &str) -> Result<&Category> {
        self.categories
            .get(category)
            .ok_or_else(|| Error::MissingCategory(category.to_owned()))
    }

    fn to_metadata(&self, category: &Category) -> run::Metadata {
        Metadata {
            game: self.name.clone(),
            category: category.name.clone(),
        }
    }

    fn to_splits(&self, category: &Category) -> Result<Vec<split::Split>> {
        let mut splits = vec![];
        for groupid in &category.groups {
            let group = self
                .groups
                .get(groupid)
                .ok_or_else(|| Error::MissingGroup(groupid.clone()))?;
            splits.extend(
                group
                    .splits
                    .iter()
                    .map(|split| split::Split::new(&split.name)),
            )
        }
        Ok(splits)
    }
}

/// A run category.
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    /// The display name of the category.
    pub name: String,
    /// The list of groups that make up the category.
    #[serde(default)]
    pub groups: Vec<GroupId>,
}

/// A configured split group.
#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    /// The name of the group.
    pub name: String,
    /// The splits inhabiting the group.
    pub splits: Vec<Split>,
}

/// A configured split.
#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    /// The split name.
    pub name: String,
    /// The set of records configured for this split.
    pub records: HashMap<CategoryId, Record>,
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

/// A group ID.
pub type GroupId = String;

/// A category ID.
pub type CategoryId = String;

/// Enumeration of errors occurring when interpreting game config.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error reading game config")]
    Io(#[from] std::io::Error),
    #[error("Error parsing game config from TOML")]
    Toml(#[from] toml::de::Error),

    /// Something referenced a missing category.
    #[error("Missing category: {0}")]
    MissingCategory(CategoryId),

    /// Something referenced a missing group.
    #[error("Missing group: {0}")]
    MissingGroup(GroupId),
}

/// Shorthand for results over [Error].
type Result<T> = std::result::Result<T, Error>;
