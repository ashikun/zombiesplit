//! Models about splits in the context of a game configuration.

use crate::model::short;

/// Info about a split.
///
/// Records here correspond to entries in the `split` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Split {
    /// The short name of this split.
    pub short: short::Name,
    /// The display name of this split.
    pub name: String,
    // TODO(@MattWindsor91): segments
}

impl Split {
    /// Constructs a new split information entry.
    ///
    /// ```
    /// use zombiesplit::model::game::split;
    ///
    /// let split = split::Split::new("pp1", "Palmtree Panic 1");
    /// assert_eq!("pp1", split.short.to_string());
    /// assert_eq!("Palmtree Panic 1", split.name);
    /// ```
    pub fn new(short: impl Into<short::Name>, name: &(impl ToString + ?Sized)) -> Self {
        Split {
            short: short.into(),
            name: name.to_string(),
        }
    }
}
