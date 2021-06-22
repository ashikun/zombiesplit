/*!
Top-level code relating to loading models from files.

Models are invariably stored in TOML files in zombiesplit, so this mostly wraps
around the serde/toml code.
*/

use std::{io::Read, path::Path};
use thiserror::Error;

pub trait Loadable: Sized {
    /// Loads this model from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file doesn't exist, or deserialisation fails.
    fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self>;
}

impl<T: serde::de::DeserializeOwned> Loadable for T {
    fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let result = toml::from_str(&contents)?;
        Ok(result)
    }
}

/// Enumeration of errors occurring when interpreting game config.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error reading game config")]
    Io(#[from] std::io::Error),
    #[error("Error parsing game config from TOML")]
    Toml(#[from] toml::de::Error),
}
/// Shorthand for a model load error.
pub type Result<T> = std::result::Result<T, Error>;
