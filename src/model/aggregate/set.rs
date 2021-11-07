//! Sets of aggregates.

use std::ops::{Index, IndexMut};

use crate::model::attempt::split;

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
    pub split: Time,
    /// Cumulative time for all splits up to and including this split.
    pub cumulative: Time,
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
        pairs: impl IntoIterator<Item = (T, Time)>,
    ) -> impl Iterator<Item = (T, Set)> {
        pairs
            .into_iter()
            .scan(Time::default(), |cumulative, (short, split)| {
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
/// use zombiesplit::model::{aggregate::{Set, Scope}, Time};
///
/// let x = Set {
///   split: Time::seconds(20).unwrap(),
///   cumulative: Time::seconds(40).unwrap()
/// };
/// assert_eq!("20s000", x[Scope::Split].to_string());
/// assert_eq!("40s000", x[Scope::Cumulative].to_string());
/// ```
impl Index<Scope> for Set {
    type Output = Time;

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
/// use zombiesplit::model::{aggregate::{Set, Scope}, Time};
///
/// let mut x = Set::default();
/// x[Scope::Split] = Time::seconds(20).unwrap();
/// x[Scope::Cumulative] = Time::seconds(40).unwrap();
///
/// assert_eq!("20s000", x[Scope::Split].to_string());
/// assert_eq!("40s000", x[Scope::Cumulative].to_string());
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
    use crate::model::{short, Time};

    #[test]
    fn accumulate_pairs_nonempty() {
        let pairs = [
            (short::Name::from("split1"), Time::seconds(63).unwrap()),
            (short::Name::from("split2"), Time::seconds(42).unwrap()),
            (short::Name::from("split3"), Time::seconds(101).unwrap()),
        ];
        let results: Vec<_> = Set::accumulate_pairs(pairs).collect();
        assert_eq!(3, results.len(), "expected as many aggregates as splits");

        let mut cumulative = Time::default();
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
