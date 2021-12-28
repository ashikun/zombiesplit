//! Split sets.
use crate::model::{aggregate, short};

use super::super::super::game;

use super::Split;

/// A set of splits, addressable by [Locator]s.
///
/// To operate on an individual split, use a [Locator] to get a reference to it,
/// and use the method on the respective [Split].
pub struct Set {
    /// The vector itself.
    contents: Vec<Split>,
    /// Lookup cache from short names to positions in the vector.
    ///
    /// Used for implementing [Locator] for short names.
    cache: short::Map<usize>,
}

/// We can construct a [Set] from any iterator that yields us split information.
impl FromIterator<game::Split> for Set {
    fn from_iter<T: IntoIterator<Item = game::Split>>(iter: T) -> Self {
        let contents: Vec<Split> = iter.into_iter().map(Split::new).collect();
        let cache = make_cache(&contents);
        Self { contents, cache }
    }
}

impl Set {
    /// Gets the number of splits in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contents.len()
    }

    /// Gets whether the split set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Wipes all data for all splits.
    pub fn reset(&mut self) {
        for s in &mut self.contents {
            s.clear();
        }
    }

    /// Gets an iterator of all of the splits in this set, in position order.
    ///
    /// The short name of each split can be found through its metadata.
    pub fn iter(&self) -> impl Iterator<Item = &Split> {
        self.contents.iter()
    }

    /// Iterates over all of the aggregate times for this split set.
    pub fn aggregates(&self) -> impl Iterator<Item = (&Split, aggregate::Set)> + '_ {
        aggregate::Set::accumulate_splits(self.contents.iter())
    }

    /// Uses `loc` to find a split in this set.
    #[must_use]
    pub fn get(&self, loc: impl Locator) -> Option<&Split> {
        loc.locate(self)
    }

    /// Uses `loc` to find a split in this set, returning a mutable reference.
    #[must_use]
    pub fn get_mut(&mut self, loc: impl Locator) -> Option<&mut Split> {
        loc.locate_mut(self)
    }
}

fn make_cache(from: &[Split]) -> short::Map<usize> {
    from.iter()
        .enumerate()
        .map(|(i, s)| (s.info.short, i))
        .collect()
}

/// Trait for things that locate a split in a set.
pub trait Locator {
    /// Gets a non-mutable reference to the given split in `set`.
    fn locate(self, set: &Set) -> Option<&Split>;

    /// Gets a mutable reference to the given split in `set`.
    fn locate_mut(self, set: &mut Set) -> Option<&mut Split>;
}

/// Locate a split by absolute position.
impl Locator for usize {
    fn locate(self, set: &Set) -> Option<&Split> {
        set.contents.get(self)
    }

    fn locate_mut(self, set: &mut Set) -> Option<&mut Split> {
        set.contents.get_mut(self)
    }
}

/// Locate a split by name.
impl Locator for short::Name {
    fn locate(self, set: &Set) -> Option<&Split> {
        set.cache.get(&self).and_then(|u| set.contents.get(*u))
    }

    fn locate_mut(self, set: &mut Set) -> Option<&mut Split> {
        let contents = &mut set.contents; // necessary for borrowck
        set.cache.get(&self).and_then(move |u| contents.get_mut(*u))
    }
}

#[cfg(test)]
mod test {
    use super::Set;
    use crate::model::{game::Split, short, Time};

    #[test]
    fn aggregates_sample_run() {
        let mut set: Set = splits().into_iter().collect();

        let s1: short::Name = "s1".into();
        let s3: short::Name = "s3".into();

        let s1s = set.get_mut(s1).expect("split 1 should exist");
        s1s.push(Time::seconds(10).unwrap());
        s1s.push(Time::seconds(25).unwrap());

        let s3s = set.get_mut(s3).expect("split 3 should exist");
        s3s.push(Time::seconds(110).unwrap());

        assert_eq!(
            3,
            set.aggregates().count(),
            "there should be exactly three aggregates"
        );

        // TODO(@MattWindsor91): investigate times
    }

    // TODO(@MattWindsor91): possibly unify this with the integration test version?
    fn splits() -> [Split; 3] {
        [
            Split::new(0, "s1", "Split 1"),
            Split::new(1, "s2", "Split 2"),
            Split::new(2, "s3", "Split 3"),
        ]
    }
}
