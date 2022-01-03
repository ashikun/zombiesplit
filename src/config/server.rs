//! Main system configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Server configuration for zombiesplit.
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    /// Address to which the server should be bound.
    pub server_addr: std::net::SocketAddr,
    /// Database location.
    pub db_path: PathBuf,
    /// The comparison provider.
    pub comparison_provider: ComparisonProvider,
}

impl<'p> Default for Server {
    fn default() -> Self {
        Self {
            server_addr: default_addr(),
            db_path: PathBuf::from("zombiesplit.db"),
            comparison_provider: ComparisonProvider::default(),
        }
    }
}

impl Server {
    /// Loads server configuration.
    ///
    /// The standard client configuration path is
    /// `(system config dir)/xyz.ashikun.zombiesplit/server.toml`.  If this file exists, it is
    /// loaded first.
    ///
    /// If `custom_path` is given, it will be loaded afterwards.
    ///
    /// # Errors
    ///
    /// Fails if we can't load any of the files needed for the configuration, or there is a problem
    /// deserialising the configuration.
    pub fn load(custom_path: Option<std::path::PathBuf>) -> Result<Self, config::ConfigError> {
        super::util::base_config("client", custom_path)?.try_into()
    }
}

/// Gets the default server binding address.
#[must_use]
pub fn default_addr() -> std::net::SocketAddr {
    std::net::SocketAddr::new(std::net::IpAddr::from([127, 0, 0, 1]), DEFAULT_PORT)
}

/// Default port for the server.
pub const DEFAULT_PORT: u16 = 1337;

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
