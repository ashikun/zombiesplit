//! Font maps, and ways to look up fonts within them.

use std::path::PathBuf;

use super::{metrics::Metrics, Result};
use serde::{Deserialize, Serialize};

/// Map from fonts to some form of configuration.
///
/// Generally, the configuration type will be [Path] for paths to font assets,
/// and `super::Metrics` for metrics maps.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Map<T> {
    /// Small text font, used for sigils and side-information.
    pub small: T,
    /// Medium text font, used for most body text.
    pub medium: T,
    /// Large text font, used for headings and standout information.
    pub large: T,
}

/// [Map]s can be indexed by [Id].
impl<T> std::ops::Index<Id> for Map<T> {
    type Output = T;

    fn index(&self, id: Id) -> &T {
        match id {
            Id::Small => &self.small,
            Id::Medium => &self.medium,
            Id::Large => &self.large,
        }
    }
}

/// [Map]s can be indexed mutably by [Id].
impl<T> std::ops::IndexMut<Id> for Map<T> {
    fn index_mut(&mut self, id: Id) -> &mut T {
        match id {
            Id::Small => &mut self.small,
            Id::Medium => &mut self.medium,
            Id::Large => &mut self.large,
        }
    }
}

impl<T> IntoIterator for Map<T> {
    type Item = (Id, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (Id::Small, self.small),
            (Id::Medium, self.medium),
            (Id::Large, self.large),
        ]
        .into_iter()
    }
}

/// A font directory path (wrapping a `PathBuf`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Path(
    // We can't use Path here because we use confy to load/store config, and that doesn't support
    // serde borrowing.
    pub std::path::PathBuf,
);

impl Path {
    /// Constructs a path from the given theme directory and font ID.
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::font::map;
    ///
    /// let mut expected = std::path::PathBuf::new();
    /// expected.push("foo");
    /// expected.push("fonts");
    /// expected.push("large");
    ///
    /// assert_eq!(expected, map::Path::new(std::path::Path::new("foo"), map::Id::Large).0);
    /// ```
    #[must_use]
    pub fn new(theme_dir: &std::path::Path, id: Id) -> Path {
        let mut pb = theme_dir.to_path_buf();
        pb.push("fonts");
        pb.push(font_dir(id));
        Path(pb)
    }

    /// Constructs the path to the font's texture.
    #[must_use]
    pub fn texture_path(&self) -> PathBuf {
        self.0.join(TEXTURE_FILE)
    }

    /// Resolves the path to the font's metrics file and tries to load it.
    ///
    /// # Errors
    ///
    /// Returns an error if the font metrics file is unreachable or unparseable
    /// as TOML.
    pub fn metrics(&self) -> Result<Metrics> {
        let str = std::fs::read_to_string(self.0.join(METRICS_FILE))?;
        Ok(toml::from_str(&str)?)
    }
}

fn font_dir(id: Id) -> &'static str {
    match id {
        Id::Small => "small",
        Id::Medium => "medium",
        Id::Large => "large",
    }
}

impl Map<Path> {
    /// Resolves metrics for all of the paths in this map.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the font metrics files are unreachable or
    /// unparseable as TOML.
    pub fn metrics(&self) -> Result<Map<Metrics>> {
        Ok(Map {
            small: self.small.metrics()?,
            medium: self.medium.metrics()?,
            large: self.large.metrics()?,
        })
    }

    /// Constructs a path map using conventional paths starting from `theme_dir`.
    #[must_use]
    pub fn new(theme_dir: &std::path::Path) -> Self {
        Self {
            small: Path::new(theme_dir, Id::Small),
            medium: Path::new(theme_dir, Id::Medium),
            large: Path::new(theme_dir, Id::Large),
        }
    }
}

/// A key in the font manager's lookup table.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Id {
    /// Small text font, used for sigils and side-information.
    Small,
    /// Medium text font, used for most body text.
    Medium,
    /// Large text font, used for headings and standout information.
    Large,
}

/// The default font ID is the medium one.
impl Default for Id {
    fn default() -> Self {
        Self::Medium
    }
}

impl Id {
    /// Constructs a `Spec` with this `Id` and the given `colour`.
    #[must_use]
    pub const fn coloured(self, colour: super::super::colour::fg::Id) -> Spec {
        Spec { id: self, colour }
    }

    /// All IDs currently available.
    pub const ALL: &'static [Self] = &[Id::Small, Id::Medium, Id::Large];
}

// A font specification (ID and colour).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Spec {
    /// The identifier of the font.
    pub id: Id,
    /// The colour key for the font.
    pub colour: super::super::colour::fg::Id,
}

/// The metrics filename.
const METRICS_FILE: &str = "metrics.toml";
/// The texture filename.
const TEXTURE_FILE: &str = "font.png";
