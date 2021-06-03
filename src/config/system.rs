//! Main system configuration.

use crate::view;
use serde::{Deserialize, Serialize};
use std::{io::Read, path::Path};
use thiserror::Error;

/// System configuration for zombiesplit.
#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    /// UI configuration.
    pub ui: view::Config,
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
