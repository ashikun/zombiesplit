//! Font sets, config, and identifiers for looking up config from a set.

use std::path::{Path, PathBuf};

use super::{metrics::Metrics, Result};
use serde::{Deserialize, Serialize};

/// Map from fonts to some form of configuration.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Map<T> {
    /// The small font, used for attempt counts.
    pub small: T,
    /// The normal font.
    pub normal: T,
    /// The large font, used for titles and totals.
    pub large: T,
}

impl<T> Map<T> {
    /// Gets the configuration for a particular font.
    #[must_use]
    pub fn get(&self, id: Id) -> &T {
        match id {
            Id::Small => &self.small,
            Id::Normal => &self.normal,
            Id::Large => &self.large,
        }
    }
}

impl<'p> Map<&'p Path> {
    /// Resolves all of the paths in this map into configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the font metrics files are unreachable or
    /// unparseable as TOML.
    pub fn resolve(&self) -> Result<Map<Config<'p>>> {
        Ok(Map {
            small: resolve_font(self.small)?,
            normal: resolve_font(self.normal)?,
            large: resolve_font(self.large)?,
        })
    }
}

fn resolve_font(path: &Path) -> Result<Config> {
    let str = std::fs::read_to_string(path.join(METRICS_FILE))?;
    let metrics = toml::from_str(&str)?;

    Ok(Config { path, metrics })
}

/// A key in the font manager's lookup table.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Id {
    /// Small font.
    Small,
    /// Normal font.
    Normal,
    /// Large font.
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

/// A fully resolved font configuration.
#[derive(Clone, Copy, Debug)]
pub struct Config<'p> {
    /// The font path.
    pub path: &'p Path,
    /// The font metrics.
    pub metrics: Metrics,
}

impl<'p> Config<'p> {
    /// Constructs the path to the font's texture.
    #[must_use]
    pub fn texture_path(&self) -> PathBuf {
        self.path.join(TEXTURE_FILE)
    }
}

/// The metrics filename.
const METRICS_FILE: &str = "metrics.toml";
/// The texture filename.
const TEXTURE_FILE: &str = "font.png";
