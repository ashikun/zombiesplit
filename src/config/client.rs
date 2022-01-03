//! Client configuration.
use serde::{Deserialize, Serialize};

use crate::ui::view;

/// Client configuration for zombiesplit.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Client {
    /// Address of the server.
    pub server_addr: std::net::SocketAddr,
    /// UI configuration.
    pub ui: view::Config,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            server_addr: super::server::default_addr(),
            ui: view::Config::default(),
        }
    }
}

impl Client {
    /// Loads client configuration.
    ///
    /// The standard client configuration path is
    /// `(system config dir)/xyz.ashikun.zombiesplit/client.toml`.  If this file exists, it is
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
