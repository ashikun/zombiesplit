//! Font identifiers.

use std::ops::{Index, IndexMut};
use std::path::Path;

use serde::{Deserialize, Serialize};

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
    pub const fn coloured(self, colour: super::colour::fg::Id) -> Spec {
        Spec { id: self, colour }
    }

    /// All IDs currently available.
    pub const ALL: &'static [Self] = &[Id::Small, Id::Medium, Id::Large];
}

serde_plain::derive_display_from_serialize!(Id);

/// Shorthand for a `zombiesplit` font spec.
pub type Spec = ugly::font::Spec<Id, super::colour::fg::Id>;

/// The font map for `zombiesplit`.
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
#[serde(default, rename_all = "kebab-case")]
pub struct Map<T> {
    /// Small text font, used for sigils and side-information.
    #[serde(default)]
    small: T,
    /// Medium text font, used for most body text.
    #[serde(default)]
    medium: T,
    /// Large text font, used for headings and standout information.
    #[serde(default)]
    large: T,
}

impl<T> Map<T> {
    fn generate(f: impl Fn(Id) -> T) -> Self {
        Self {
            small: f(Id::Small),
            medium: f(Id::Medium),
            large: f(Id::Large),
        }
    }
}

impl<T> Index<Id> for Map<T> {
    type Output = T;

    fn index(&self, index: Id) -> &Self::Output {
        match index {
            Id::Small => &self.small,
            Id::Medium => &self.medium,
            Id::Large => &self.large,
        }
    }
}

impl<T> IndexMut<Id> for Map<T> {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        match index {
            Id::Small => &mut self.small,
            Id::Medium => &mut self.medium,
            Id::Large => &mut self.large,
        }
    }
}

impl<T> ugly::resource::Map<T> for Map<T> {
    type Id = Id;

    fn get(&self, k: Self::Id) -> &T {
        self.index(k)
    }
}

impl<T> ugly::resource::MutableMap<T> for Map<T> {
    fn set(&mut self, k: Self::Id, v: T) {
        *self.index_mut(k) = v
    }
}

impl ugly::font::Map for Map<ugly::Font> {
    type MetricsMap = Map<ugly::font::Metrics>;
    type IndexMap = Map<ugly::font::Index>;

    fn load_metrics(&self) -> ugly::font::Result<Self::MetricsMap> {
        Ok(Self::MetricsMap {
            small: self.small.metrics()?,
            medium: self.medium.metrics()?,
            large: self.large.metrics()?,
        })
    }
}

impl Map<Option<ugly::Font>> {
    /// Constructs a path map rooted at `assets_path`.
    ///
    /// Each path is optional; it will be `None` if the font path doesn't resolve to a font.
    /// Use `require` to get a map of definite fonts.
    #[must_use]
    pub fn pathmap(assets_path: &Path) -> Self {
        Self::generate(|id| path(id, assets_path))
    }

    /// Constructs a partial path map rooted at `assets_path`.
    ///
    /// # Errors
    ///
    /// Fails if any of the fonts are `None`.
    pub fn require(self) -> Result<Map<ugly::Font>> {
        Ok(Map {
            small: self.small.ok_or(Error::MissingFont(Id::Small))?,
            medium: self.medium.ok_or(Error::MissingFont(Id::Medium))?,
            large: self.large.ok_or(Error::MissingFont(Id::Large))?,
        })
    }

    pub fn merge(&mut self, mut other: Self) {
        for i in [Id::Small, Id::Medium, Id::Large] {
            if let Some(v) = other.index_mut(i).take() {
                self[i] = Some(v);
            }
        }
    }
}

fn path(id: Id, assets_path: &Path) -> Option<ugly::font::Font> {
    let path = assets_path.join("fonts").join(id.to_string());
    path.is_dir().then(|| ugly::Font::from_dir(path))
}

/// Errors that can be returned from font operations.
#[derive(Debug, Copy, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The given font was missing during a `require`.
    #[error("Missing font: {0}")]
    MissingFont(Id),
}

/// Shorthand for results over [Error].
pub type Result<T> = std::result::Result<T, Error>;
