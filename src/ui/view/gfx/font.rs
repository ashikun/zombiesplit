//! Font identifiers.

use serde::{Deserialize, Serialize};
use std::path::Path;

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

impl ugly::font::Id for Id {}

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

/// Shorthand for a `zombiesplit` font path map.
pub type PathMap = ugly::font::path::Map<Id>;

/// Constructs a path map rooted at `assets_path`.
#[must_use]
pub fn pathmap(assets_path: &Path) -> PathMap {
    Id::ALL
        .iter()
        .map(|i| {
            (
                *i,
                ugly::font::Path(assets_path.join("fonts").join(i.to_string())),
            )
        })
        .collect()
}
