//! Path and miscellaneous other utilities for configuration.

use config::Config;
use std::path::PathBuf;

/// Gets the base config from folding in the user's custom file and the standard file (if any).
pub(super) fn base_config(
    name: &str,
    custom_path: Option<std::path::PathBuf>,
) -> Result<config::Config> {
    let mut s = config::Config::new();

    merge_file(&mut s, "global", name, global_config_path(name), false)?;
    merge_file(&mut s, "user", name, custom_path, true)?;

    Ok(s)
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
    s: &mut Config,
    scope: &str,
    name: &str,
    path: Option<PathBuf>,
    is_required: bool,
) -> Result<()> {
    if let Some(path) = path {
        log::info!("Using {scope} {name} config file: {path:?}");
        s.merge(config::File::from(path).required(is_required))?;
    }

    Ok(())
}

type Result<T> = std::result::Result<T, config::ConfigError>;
