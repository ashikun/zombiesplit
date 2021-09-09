//! Models about splits in the context of a game configuration.

use crate::model::short;

/// Info about a split.
///
/// Records here correspond to entries in the `split` table.
#[derive(Debug)]
pub struct Split {
    /// The ID of this split in the table.
    pub id: i64,
    /// The short name of this split.
    pub short: short::Name,
    /// The display name of this split.
    pub name: String,
    // TODO(@MattWindsor91): segments
}
