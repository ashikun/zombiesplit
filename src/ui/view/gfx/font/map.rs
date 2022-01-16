//! Font maps, and ways to look up fonts within them.

use std::path::PathBuf;

use super::{metrics::Metrics, Result};
use serde::{Deserialize, Serialize};

/// Map from fonts to some form of configuration.
///
/// Generally, the configuration type will be [Path] for paths to font assets,
/// and `super::Metrics` for metrics maps.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Map<T> {
    /// Small text font, used for sigils and side-information.
    pub small: T,
    /// Medium text font, used for most body text.
    pub medium: T,
    /// Large text font, used for headings and standout information.
    pub large: T,
}

/// The default path map assumes the fonts are in 'assets/fonts'.
impl Default for Map<Path> {
    fn default() -> Self {
        Self {
            small: Path::new("assets/fonts/small"),
            medium: Path::new("assets/fonts/medium"),
            large: Path::new("assets/fonts/large"),
        }
    }
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Path(
    // We can't use Path here because we use confy to load/store config, and that doesn't support
    // serde borrowing.
    std::path::PathBuf,
);

impl Path {
    /// Constructs a path from the given string.
    #[must_use]
    pub fn new(raw: &str) -> Path {
        use std::str::FromStr;
        match std::path::PathBuf::from_str(raw) {
            Ok(x) => Path(x),
            Err(e) => match e {},
        }
    }

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

impl Map<Path> {
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
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Id {
    /// Small text font, used for sigils and side-information.
    Small,
    /// Medium text font, used for most body text.
    Medium,
    /// Large text font, used for headings and standout information.
    Large,
}

/// The default font ID is the medium one.
impl Default for Id {
    fn default() -> Self {
        Self::Medium
    }
}

impl Id {
    /// Constructs a `Spec` with this `Id` and the given `colour`.
    #[must_use]
    pub fn coloured(self, colour: super::super::colour::fg::Id) -> Spec {
        Spec { id: self, colour }
    }
}

// A font specification (ID and colour).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
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
