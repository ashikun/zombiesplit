//! Sets of aggregates.

use std::ops::{Index, IndexMut};

use crate::model::{attempt::split, short};

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

impl Set {
    /// Constructs an iterator that yields, for each split in `splits`, the
    /// split's aggregate times.
    ///
    /// Although split aggregates are yielded in order, we provide the short
    /// name of each split for convenience.
    pub fn accumulate<'a>(
        splits: impl IntoIterator<Item = &'a split::Split> + 'a,
    ) -> impl Iterator<Item = (short::Name, Set)> + 'a {
        Self::accumulate_pairs(
            splits
                .into_iter()
                .map(|split| (split.info.short, split.total_time())),
        )
    }

    /// Constructs an iterator that yields, for each pair of short-name and
    /// total time in `pairs`, a corresponding stream of aggregate times.
    ///
    /// Although aggregates are yielded in order, we provide the short
    /// name of each split for convenience.
    pub fn accumulate_pairs(
        pairs: impl IntoIterator<Item = (short::Name, Time)>,
    ) -> impl Iterator<Item = (short::Name, Set)> {
        pairs
            .into_iter()
            .scan(Time::default(), |cumulative, (short, split)| {
                *cumulative += split;
                let agg = Set {
                    split: Some(split),
                    cumulative: Some(*cumulative),
                };

                Some((short, agg))
            })
    }
}

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
