//! Configuration structs for split groups, splits, records, and categories.
pub mod time;
use time::Time;

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    str::FromStr,
};

/// A configured split group.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Map of split groups for the game.
    pub groups: HashMap<GroupId, Group>,
    /// Map of categories for the game.
    pub categories: HashMap<CategoryId, Category>,
}

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
    pub time: Time,
}

impl Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time)
    }
}

impl FromStr for Record {
    type Err = time::ParseError; // for now

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Record { time: s.parse()? })
    }
}

/// A group ID.
pub type GroupId = String;

/// A category ID.
pub type CategoryId = String;
