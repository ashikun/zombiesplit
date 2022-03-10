//! Client configuration.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::ui::view;

/// Client configuration for zombiesplit.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Client {
    /// Address of the server.
    pub server_addr: String,
    /// UI configuration.
    pub ui: Ui,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            server_addr: format!("https://localhost:{}", super::server::DEFAULT_PORT),
            ui: Ui::default(),
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
        super::util::base_config("client", custom_path)
            .build()?
            .try_deserialize()
    }
}

/// User interface configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Ui {
    /// Layout configuration for the user interface.
    pub layout: view::config::Layout,

    /// Path to a theme that will be loaded on top of the base assets.
    pub theme: Option<std::path::PathBuf>,
}

impl Ui {
    /// Resolves this configuration into a `view::Config`, primarily by loading the theme data.
    /// # Errors
    ///
    /// Fails if we couldn't load in the theme correctly.
    pub fn into_view_config(self) -> view::config::theme::Result<view::Config> {
        let theme = resolve_theme(self.theme)?;
        Ok(view::Config {
            theme,
            layout: self.layout,
        })
    }
}

fn resolve_theme(
    path: Option<std::path::PathBuf>,
) -> view::config::theme::Result<view::config::Theme> {
    let base_path = base_theme_path();
    path.map_or_else(
        || view::config::Theme::base(&base_path),
        |theme_path| {
            view::config::theme::LoadPathset {
                base: &base_path,
                theme: &theme_path,
            }
            .load()
        },
    )
}

fn base_theme_path() -> std::path::PathBuf {
    let mut path = super::util::dir().map_or_else(PathBuf::new, |d| d.data_dir().to_path_buf());
    path.push("assets");
    path
}
