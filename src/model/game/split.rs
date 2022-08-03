//! Models about splits in the context of a game configuration.

use crate::model::short;

/// Information about a segment.
///
/// Records here correspond to entries in the `split` table.  (There is currently a terminology
/// mismatch; we are phasing out the term 'split' in favour of 'segment'.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Split {
    /// The short name of this segment.
    pub short: short::Name,
    /// The display name of this segment.
    pub name: String,
    /// The nickname of this segment.
    pub nickname: Option<String>,
    // TODO(@MattWindsor91): segments
}

impl Split {
    /// Constructs a new segment.
    ///
    /// The split will initially have an empty nickname.
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
            nickname: None,
        }
    }
}
