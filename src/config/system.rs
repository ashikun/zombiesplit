//! Main system configuration.

use crate::ui::view;
use serde::{Deserialize, Serialize};
use std::{
    io::Read,
    path::{Path, PathBuf},
};
use thiserror::Error;

/// System configuration for zombiesplit.
#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    /// Database location.
    pub db_path: PathBuf,
    /// UI configuration.
    pub ui: view::Config,
    /// The comparison provider.
    pub comparison_provider: ComparisonProvider,
}

/// Enumerates the various up-front ways in which zombiesplit knows to source
/// a comparison.
///
/// New methods may be added to this in future.  In addition, the lower-level
/// zombiesplit API is open to any provider that implements the appropriate
/// trait.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ComparisonProvider {
    /// Don't compare against anything.
    None,
    /// Compare against the PB run in the database.
    Database,
}

/// By default, there are no comparisons.
impl Default for ComparisonProvider {
    fn default() -> Self {
        Self::None
    }
}

impl System {
    /// Loads system config from TOML.
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

/// Enumeration of errors occurring when interpreting system config.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error reading system config")]
    Io(#[from] std::io::Error),
    #[error("Error parsing system config from TOML")]
    Toml(#[from] toml::de::Error),
}

/// Shorthand for results over [Error].
type Result<T> = std::result::Result<T, Error>;
