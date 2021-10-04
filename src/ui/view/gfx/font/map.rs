//! Font maps, and ways to look up fonts within them.

use std::path::PathBuf;

use super::{metrics::Metrics, Result};
use serde::{Deserialize, Serialize};

/// Map from fonts to some form of configuration.
///
/// Generally, the configuration type will be [Path] for paths to font assets,
/// and `super::Metrics` for metrics maps.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Map<T> {
    /// Small text font, used for sigils and side-information.
    pub small: T,
    /// Medium text font, used for most body text.
    pub medium: T,
    /// Large text font, used for headings and standout information.
    pub large: T,
}

/// [Map]s can be indexed by [Id].
impl<T> std::ops::Index<Id> for Map<T> {
    type Output = T;

    fn index(&self, id: Id) -> &T {
        match id {
            Id::Small => &self.small,
            Id::Medium => &self.medium,
            Id::Large => &self.large,
        }
    }
}

/// A font directory path.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Path<'p>(#[serde(borrow)] &'p std::path::Path);

impl<'p> Path<'p> {
    /// Constructs the path to the font's texture.
    #[must_use]
    pub fn texture_path(&self) -> PathBuf {
        self.0.join(TEXTURE_FILE)
    }

    /// Resolves the path to the font's metrics file and tries to load it.
    ///
    /// # Errors
    ///
    /// Returns an error if the font metrics file is unreachable or unparseable
    /// as TOML.
    pub fn metrics(&self) -> Result<Metrics> {
        let str = std::fs::read_to_string(self.0.join(METRICS_FILE))?;
        Ok(toml::from_str(&str)?)
    }
}

impl<'p> Map<Path<'p>> {
    /// Resolves metrics for all of the paths in this map.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the font metrics files are unreachable or
    /// unparseable as TOML.
    pub fn metrics(&self) -> Result<Map<Metrics>> {
        Ok(Map {
            small: self.small.metrics()?,
            medium: self.medium.metrics()?,
            large: self.large.metrics()?,
        })
    }
}

/// A key in the font manager's lookup table.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Id {
    /// Small text font, used for sigils and side-information.
    Small,
    /// Medium text font, used for most body text.
    Medium,
    /// Large text font, used for headings and standout information.
    Large,
}

// A font specification (ID and colour).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Spec {
    /// The identifier of the font.
    pub id: Id,
    /// The colour key for the font.
    pub colour: super::super::colour::fg::Id,
}

/// The metrics filename.
const METRICS_FILE: &str = "metrics.toml";
/// The texture filename.
const TEXTURE_FILE: &str = "font.png";