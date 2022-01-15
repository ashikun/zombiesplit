//! Path and miscellaneous other utilities for configuration.

/// Gets the path for the configuration file with name `name`.
pub fn get(name: &str) -> Option<std::path::PathBuf> {
    dir().map(|d| {
        let mut pb = std::path::PathBuf::from(d.config_dir());

        pb.push(format!("{name}.toml"));

        pb
    })
}

fn dir() -> Option<directories::ProjectDirs> {
    // TODO(@MattWindsor91): maybe eventually there will be a better organisation here.
    directories::ProjectDirs::from("xyz", "ashikun", "zombiesplit")
}

/// Gets the base config from folding in the user's custom file and the standard file (if any).
pub(super) fn base_config(
    name: &str,
    custom_path: Option<std::path::PathBuf>,
) -> Result<config::Config, config::ConfigError> {
    let mut s = config::Config::new();

    if let Some(path) = get(name) {
        // The validity of the path doesn't necessarily mean the config file actually exists.
        log::info!("Using main client config file: {path:?}");
        s.merge(config::File::from(path).required(false))?;
    }

    if let Some(path) = custom_path {
        log::info!("Using user config file: {path:?}");
        s.merge(config::File::from(path).required(true))?;
    }

    Ok(s)
}
