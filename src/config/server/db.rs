//! Database configuration for the server.
use super::super::util;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Server configuration for the database.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct Database {
    /// The database location.
    pub path: PathBuf,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: default_path(),
        }
    }
}

fn default_path() -> PathBuf {
    let mut path = util::dir().map_or_else(PathBuf::new, |d| PathBuf::from(d.data_dir()));
    path.push(DB_FILE);
    path
}

/// The default database file, relative to the zombiesplit data directory.
const DB_FILE: &str = "zombiesplit.db";
