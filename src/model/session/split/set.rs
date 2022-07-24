//! Split sets.
use itertools::{Either, Itertools};

use super::{
    super::super::{game, short, timing::aggregate},
    Split,
};

/// A set of splits, addressable by [Locator]s.
///
/// To operate on an individual split, use a [Locator] to get a reference to it,
/// and use the method on the respective [Split].
#[derive(Debug, Clone)]
pub struct Set {
    /// The vector itself.
    contents: Vec<Split>,
    /// Lookup cache from short names to positions in the vector.
    ///
    /// Used for implementing [Locator] for short names.
    cache: short::Map<usize>,
}

/// We can construct a [Set] from any iterator that yields attempt split information.
impl FromIterator<Split> for Set {
    fn from_iter<T: IntoIterator<Item = Split>>(iter: T) -> Self {
        let contents: Vec<Split> = iter.into_iter().collect();
        let cache = make_cache(&contents);
        Self { contents, cache }
    }
}

/// We can construct a [Set] from any iterator that yields initial (game) split information.
/// The result is as if we created an empty attempt split for each game split first.
impl FromIterator<game::Split> for Set {
    fn from_iter<T: IntoIterator<Item = game::Split>>(iter: T) -> Self {
        iter.into_iter().map(Split::new).collect()
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

    /// Constructs a split set from the information in both `game` and `category` configuration.
    ///
    /// # Errors
    ///
    /// Fails if any of the references between configuration elements are broken.
    pub fn from_config(
        game: &game::Config,
        category: &game::config::Category,
    ) -> game::config::Result<Self> {
        category
            .full_segments(game)
            .flat_map(|r| process_segment_result(r, game))
            .map_ok(|(n, s)| game::Split::new(n, &s.name))
            .collect()
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

    /// Gets the last split that has a defined time.
    ///
    /// The cumulative time total of this split is effectively the total time of the whole attempt.
    #[must_use]
    pub fn last_entered(&self) -> Option<&Split> {
        self.contents.iter().rfind(|s| !s.times.is_empty())
    }
}

fn process_segment_result<'g>(
    r: game::config::Result<(short::Name, &'g game::config::Segment)>,
    game: &'g game::Config,
) -> impl Iterator<Item = game::config::Result<(short::Name, &'g game::config::Split)>> {
    match r {
        Ok((_, seg)) => Either::Left(seg.full_splits(game)),
        Err(e) => Either::Right(std::iter::once(Err(e))),
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
            Split::new("s1", "Split 1"),
            Split::new("s2", "Split 2"),
            Split::new("s3", "Split 3"),
        ]
    }
}
