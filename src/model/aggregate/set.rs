//! Sets of aggregates.

use std::ops::{Index, IndexMut};

use super::{
    super::Time,
    index::{Scope, Source},
};

/// A full set of aggregate times for one split, organised by source.
#[derive(Debug, Default, Clone, Copy)]
pub struct Full {
    /// Times for the current attempt.
    pub attempt: Set,
    /// Times for the comparison.
    pub comparison: Set,
}

/// [Full] sets can be indexed by [Source], yielding the [Set] for that source.
impl Index<Source> for Full {
    type Output = Set;

    fn index(&self, index: Source) -> &Self::Output {
        match index {
            Source::Attempt => &self.attempt,
            Source::Comparison => &self.comparison,
        }
    }
}

/// [Full] sets can be mutably indexed by [Source], yielding the [Set] for that
/// source.
impl IndexMut<Source> for Full {
    fn index_mut(&mut self, index: Source) -> &mut Self::Output {
        match index {
            Source::Attempt => &mut self.attempt,
            Source::Comparison => &mut self.comparison,
        }
    }
}

/// A set of aggregate times at various scopes.
#[derive(Debug, Default, Clone, Copy)]
pub struct Set {
    /// Single time for this split only.
    pub split: Option<Time>,
    /// Cumulative time for all splits up to and including this split.
    pub cumulative: Option<Time>,
}

impl Set {}

/// We can index a [Set] by scope, yielding a possible time.
impl Index<Scope> for Set {
    type Output = Option<Time>;

    fn index(&self, index: Scope) -> &Self::Output {
        match index {
            Scope::Cumulative => &self.cumulative,
            Scope::Split => &self.split,
        }
    }
}

/// We can mutably index a [Set] by scope, yielding a time slot.
impl IndexMut<Scope> for Set {
    fn index_mut(&mut self, index: Scope) -> &mut Self::Output {
        match index {
            Scope::Cumulative => &mut self.cumulative,
            Scope::Split => &mut self.split,
        }
    }
}
