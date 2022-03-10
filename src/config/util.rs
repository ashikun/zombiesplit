//! Path and miscellaneous other utilities for configuration.

use config::ConfigBuilder;
use std::path::PathBuf;

/// Gets the base config from folding in the user's custom file and the standard file (if any).
pub(super) fn base_config(
    name: &str,
    custom_path: Option<std::path::PathBuf>,
) -> ConfigBuilder<config::builder::DefaultState> {
    let s = config::ConfigBuilder::default();
    let s = merge_file(s, "global", name, global_config_path(name), false);
    merge_file(s, "user", name, custom_path, true)
}

/// Gets the path for the configuration file with name `name`.
fn global_config_path(name: &str) -> Option<std::path::PathBuf> {
    dir().map(|d| {
        let mut pb = std::path::PathBuf::from(d.config_dir());

        pb.push(format!("{name}.toml"));

        pb
    })
}

/// Gets the base project directory configuration for zombiesplit.
pub fn dir() -> Option<directories::ProjectDirs> {
    // TODO(@MattWindsor91): maybe eventually there will be a better organisation here.
    directories::ProjectDirs::from("xyz", "ashikun", "zombiesplit")
}

fn merge_file(
    s: ConfigBuilder<config::builder::DefaultState>,
    scope: &str,
    name: &str,
    path: Option<PathBuf>,
    is_required: bool,
) -> ConfigBuilder<config::builder::DefaultState> {
    if let Some(path) = path {
        log::info!("Using {scope} {name} config file: {path:?}");
        s.add_source(config::File::from(path).required(is_required))
    } else {
        s
    }
}
