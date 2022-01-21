//! Comparison configuration for the server.

use serde::{Deserialize, Serialize};

/// Server configuration for comparisons.
#[derive(Copy, Clone, Serialize, Deserialize, Default, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct Comparison {
    /// The configured provider for comparisons.
    pub provider: Provider,
}

/// Enumerates the various up-front ways in which zombiesplit knows to source
/// a comparison.
///
/// New methods may be added to this in future.  In addition, the lower-level
/// zombiesplit API is open to any provider that implements the appropriate
/// trait.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Provider {
    /// Don't compare against anything.
    None,
    /// Compare against the PB run in the database.
    Database,
}

/// By default, there are no comparisons.
impl Default for Provider {
    fn default() -> Self {
        Self::None
    }
}
