//! Sets of aggregates.

use std::ops::{Index, IndexMut};

use super::{
    super::{super::session::split, time},
    index::{Kind, Scope, Source},
};

/// A full set of aggregate times for one split, organised by source.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
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

/// [Full] sets can be indexed by [Kind], yielding the time.
impl Index<Kind> for Full {
    type Output = time::Time;

    fn index(&self, index: Kind) -> &Self::Output {
        &self[index.source][index.scope]
    }
}

/// [Full] sets can be mutably indexed by [Kind], yielding time.
impl IndexMut<Kind> for Full {
    fn index_mut(&mut self, index: Kind) -> &mut Self::Output {
        &mut self[index.source][index.scope]
    }
}

/// A set of aggregate times at various scopes.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Set {
    /// Single time for this split only.
    pub split: time::Time,
    /// Cumulative time for all splits up to and including this split.
    pub cumulative: time::Time,
}

impl Set {
    /// Constructs an iterator that yields, for each split in `splits`, the
    /// split's aggregate times.
    ///
    /// Although split aggregates are yielded in order, we provide the short
    /// name of each split for convenience.
    pub fn accumulate_splits<'a>(
        splits: impl IntoIterator<Item = &'a split::Split> + 'a,
    ) -> impl Iterator<Item = (&'a split::Split, Set)> + 'a {
        Self::accumulate_pairs(splits.into_iter().map(|x| (x, x.total_time())))
    }

    /// Constructs an iterator that yields, for each pair of split and
    /// total time in `pairs`, a corresponding stream of aggregate times.
    ///
    /// Although aggregates are yielded in order, we provide the short
    /// name of each split for convenience.
    pub fn accumulate_pairs<T>(
        pairs: impl IntoIterator<Item = (T, time::Time)>,
    ) -> impl Iterator<Item = (T, Set)> {
        pairs
            .into_iter()
            .scan(time::Time::default(), |cumulative, (short, split)| {
                *cumulative += split;
                Some((
                    short,
                    Set {
                        split,
                        cumulative: *cumulative,
                    },
                ))
            })
    }
}

/// We can index a [Set] by scope, yielding a time.
///
/// ```
/// use zombiesplit::model::timing::{aggregate::{Set, Scope}, time};
///
/// let x = Set {
///   split: time::Time::from_millis(20),
///   cumulative: time::Time::from_millis(40)
/// };
/// assert_eq!(20, x[Scope::Split].to_millis());
/// assert_eq!(40, x[Scope::Cumulative].to_millis());
/// ```
impl Index<Scope> for Set {
    type Output = time::Time;

    fn index(&self, index: Scope) -> &Self::Output {
        match index {
            Scope::Cumulative => &self.cumulative,
            Scope::Split => &self.split,
        }
    }
}

/// We can mutably index a [Set] by scope, yielding access to the time.
///
/// ```
/// use zombiesplit::model::timing::{aggregate::{Set, Scope}, time};
///
/// let mut x = Set::default();
/// x[Scope::Split] = time::Time::from_millis(20);
/// x[Scope::Cumulative] = time::Time::from_millis(40);
///
/// assert_eq!(20, x[Scope::Split]);
/// assert_eq!(40, x[Scope::Cumulative]);
/// ```
impl IndexMut<Scope> for Set {
    fn index_mut(&mut self, index: Scope) -> &mut Self::Output {
        match index {
            Scope::Cumulative => &mut self.cumulative,
            Scope::Split => &mut self.split,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Scope, Set};
    use crate::model::{short, timing::time};

    #[test]
    fn accumulate_pairs_nonempty() {
        let pairs = [
            (short::Name::from("split1"), time::Time::from_millis(63)),
            (short::Name::from("split2"), time::Time::from_millis(42)),
            (short::Name::from("split3"), time::Time::from_millis(101)),
        ];
        let results: Vec<_> = Set::accumulate_pairs(pairs).collect();
        assert_eq!(3, results.len(), "expected as many aggregates as splits");

        let mut cumulative = time::Time::default();
        for ((orig_name, orig_time), (agg_name, agg)) in pairs.iter().zip(&results) {
            assert_eq!(
                orig_name, agg_name,
                "aggregates should mention same splits in order"
            );
            assert_eq!(
                *orig_time,
                agg[Scope::Split],
                "aggregate split times should match"
            );

            cumulative += *orig_time;
            assert_eq!(
                cumulative,
                agg[Scope::Cumulative],
                "aggregate cumulative times should match"
            );
        }
    }
}
