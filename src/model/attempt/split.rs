//! Splits and related items.

use crate::model::{aggregate, short};

use super::super::{game, time::Time};
use std::iter::FromIterator;

/// A set of splits, addressable by [Locator]s.
///
/// To operate on an individual split, use a [Locator] to get a reference to it,
/// and use the method on the respective [Split].
pub struct Set {
    /// The vector itself.
    contents: Vec<Split>,
    /// Lookup cache from short names to positions in the vector.
    ///
    /// Used for implementing [Locaotr] for short names.
    cache: short::Map<usize>,
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

/// We can construct a [Set] from any iterator that yields us split data.
impl FromIterator<game::Split> for Set {
    // TODO(@MattWindsor91): remove the use of Rc here.

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

    /// Gets an iterator of all of the splits in this set, in position order,
    /// that are either `split` or will depend in some way on `split`.
    ///
    /// The short name of each split can be found through its metadata.
    pub fn dependents_of(&self, split: short::Name) -> impl Iterator<Item = &Split> {
        let pos = self.cache.get(&split).copied().unwrap_or_default();
        self.iter().skip(pos)
    }

    /// Iterates over all of the aggregate times for this split set.
    pub fn aggregates(&self) -> impl Iterator<Item = (short::Name, aggregate::Pair)> + '_ {
        self.contents
            .iter()
            .scan(Time::default(), |cumulative, split| {
                let short = split.info.short;
                // TODO(@MattWindsor91): cache the sum somewhere?
                let split = split.times.iter().copied().sum();
                *cumulative += split;
                let agg = aggregate::Pair {
                    split: Some(split),
                    cumulative: Some(*cumulative),
                };

                Some((short, agg))
            })
    }
}

fn make_cache(from: &[Split]) -> short::Map<usize> {
    from.iter()
        .enumerate()
        .map(|(i, s)| (s.info.short, i))
        .collect()
}

/// A split in a run attempt.
#[derive(Debug)]
pub struct Split {
    /// The game/category split information for this split.
    ///
    /// This contains the split's shortname, its default display name, and other
    /// such information.
    pub info: game::Split,
    /// The entered times.
    /// Invariant: none of the times are zero.
    times: Vec<Time>,
}

impl Split {
    /// Creates a new split with the given metadata and an empty time.
    #[must_use]
    pub fn new(info: game::Split) -> Self {
        Self {
            info,
            times: Vec::new(),
        }
    }

    /// Borrows this split's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.info.name
    }

    /// Clones a copy of the times for this split.
    #[must_use]
    pub fn all_times(&self) -> Vec<Time> {
        self.times.clone()
    }

    /// Gets the number of times logged for this split.
    #[must_use]
    pub fn num_times(&self) -> usize {
        self.times.len()
    }

    /// Pushes a time onto this split.
    ///
    /// If the time is zero, it will not be added.
    pub fn push(&mut self, time: Time) {
        if !time.is_zero() {
            self.times.push(time);
        }
    }

    /// Tries to pop the most recently added time off this split.
    #[must_use]
    pub fn pop(&mut self) -> Option<Time> {
        self.times.pop()
    }

    /// Removes all times from this split.
    pub fn clear(&mut self) {
        self.times.clear();
    }
}
