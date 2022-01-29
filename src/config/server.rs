//! Main system configuration.

pub mod comparison;
mod db;

use db::Database;
use serde::{Deserialize, Serialize};

/// Server configuration for zombiesplit.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Server {
    /// The comparison configuration.
    pub comparison: comparison::Comparison,
    /// Database configuration.
    pub db: Database,
    /// Network configuration.
    pub net: Net,
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
        super::util::base_config("server", custom_path)?.try_into()
    }
}

/// Server network configuration.
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct Net {
    /// Address to which the server should be bound.
    pub address: std::net::SocketAddr,
}

impl Default for Net {
    fn default() -> Self {
        Self {
            address: default_addr(),
        }
    }
}

/// Gets the default server binding address.
#[must_use]
pub fn default_addr() -> std::net::SocketAddr {
    std::net::SocketAddr::new(std::net::IpAddr::from([127, 0, 0, 1]), DEFAULT_PORT)
}

/// Default port for the server.
pub const DEFAULT_PORT: u16 = 1337;
