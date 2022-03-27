//! Theme support for the zombiesplit UI.
//!
//! A theme consists of a directory containing a palette and a set of fonts, which override the
//! default theme.

use std::path::{Path, PathBuf};
use thiserror::Error;

use super::super::gfx::{colour, font};

/// Theme configuration.
#[derive(Clone, Debug)]
pub struct Theme {
    /// Colour set.
    pub colours: colour::Palette,
    /// Font path configuration.
    pub font_paths: font::Map,
}

impl Theme {
    /// Constructs the base theme, with no palette or font overrides.
    ///
    /// `asset_path` should point to the place in the zombiesplit install where the theme is
    /// installed.
    ///
    /// # Errors
    ///
    /// Fails if the base assets are not reachable from `asset_path`, or are malformed.
    pub fn base(asset_path: &std::path::Path) -> Result<Self> {
        Self::new(colour::defaults(), &font::pathmap(asset_path))
    }

    /// Constructs a theme with `colours`, `font_paths`, and the metrics loadable via `font_paths`.
    ///
    /// # Errors
    ///
    /// Fails if the font metrics are not available for the fonts at `font_paths`.
    pub fn new(colours: colour::Palette, font_paths: &font::Map) -> Result<Self> {
        Ok(Theme {
            colours,
            font_paths: font_paths.clone(),
        })
    }
}

/// Bundles paths used in loading theme assets.
#[derive(Copy, Clone, Debug)]
pub struct LoadPathset<'p> {
    /// Path to base theme assets (such as fonts).
    pub base: &'p std::path::Path,
    /// Path to this theme's own assets.
    pub theme: &'p std::path::Path,
}

impl<'p> LoadPathset<'p> {
    /// Loads a theme using this pathset.
    ///
    /// # Errors
    ///
    /// Fails if any part of the theme (colours, fonts, etc) is missing (and is not optional), or
    /// is unparseable.
    pub fn load(&self) -> Result<Theme> {
        if self.theme.is_dir() {
            Theme::new(self.load_colours()?, &self.resolve_font_paths())
        } else {
            Err(Error::NotADir(self.theme.to_path_buf()))
        }
    }

    fn load_colours(&self) -> Result<colour::Palette> {
        let mut palette = colour::defaults();
        palette.merge(&self.load_user_colours()?);
        Ok(palette)
    }

    fn load_user_colours(&self) -> Result<colour::Palette> {
        let palette_file = theme_file(self.theme, "palette.ron");
        Ok(if palette_file.is_file() {
            colour::load_ron(palette_file)?
        } else {
            colour::Palette::default()
        })
    }

    fn resolve_font_paths(&self) -> font::Map {
        let mut paths = font::pathmap(self.base);
        let theme_paths = font::pathmap(self.theme);
        paths.extend(theme_paths);
        paths
    }
}

fn theme_file(theme_dir: &Path, element: &str) -> PathBuf {
    let mut colour_dir = theme_dir.to_path_buf();
    colour_dir.push(element);
    colour_dir
}

/// Error while loading a theme.
#[derive(Debug, Error)]
pub enum Error {
    /// The theme path does not point to a directory.
    #[error("No directory at the given theme path")]
    NotADir(std::path::PathBuf),
    /// The palette could not be deserialised.
    #[error("Could not load palette")]
    BadPalette(#[from] colour::Error),
    /// One of the fonts could not be loaded.
    #[error("Could not load font")]
    BadFont(#[from] ugly::font::Error),
}

/// Shorthand for results over [Error].
pub type Result<T> = std::result::Result<T, Error>;
