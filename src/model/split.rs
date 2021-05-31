//! Splits and related items.

use super::time::Time;

/// A split in a run.
pub struct Split {
    /// The name of the split.
    pub name: String,
    /// The entered times.
    /// Invariant: none of the times are zero.
    times: Vec<Time>,
}

impl Split {
    /// Creates a new split with the given name and an empty time.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            times: Vec::new(),
        }
    }

    /// Calculates the summed time of the split.
    #[must_use]
    pub fn summed_time(&self) -> Time {
        self.times.iter().copied().sum()
    }

    /// Gets whether this split has times registered.
    #[must_use]
    pub fn has_times(&self) -> bool {
        !self.times.is_empty()
    }

    /// Pushes a time onto this split.
    ///
    /// If the time is zero, it will not be added.
    pub fn push(&mut self, time: Time) {
        if !time.is_zero() {
            self.times.push(time)
        }
    }

    /// Tries to pop the most recently added time off this split.
    #[must_use]
    pub fn pop(&mut self) -> Option<Time> {
        self.times.pop()
    }

    /// Removes all times from this split.
    pub fn clear(&mut self) {
        self.times.clear()
    }
}

/// Possible paces for a split or run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pace {
    /// There is no pacing time.
    Inconclusive,
    /// This split, or run, is behind its comparison.
    Behind,
    /// This split, or run, is ahead its comparison.
    Ahead,
    /// This split, or run, is a personal best ('gold split').
    PersonalBest,
}

/// The default pace is inconclusive.
impl Default for Pace {
    fn default() -> Self {
        Self::Inconclusive
    }
}

/// Split comparisons.
pub struct Comparison {
    /// The personal best for this split, if any.
    pub split: Option<Time>,
    /// The time for this split in the comparison run, if any.
    pub in_run: Option<Time>,
}

impl Comparison {
    /// Compares `split_time` against this comparison.
    #[must_use]
    pub fn pace(&self, split_time: Time) -> Pace {
        if self.is_pb(split_time) {
            Pace::PersonalBest
        } else {
            self.pace_from_run(split_time)
        }
    }

    fn pace_from_run(&self, split_time: Time) -> Pace {
        self.in_run.map_or(Pace::default(), |cmp| {
            if split_time <= cmp {
                Pace::Ahead
            } else {
                Pace::Behind
            }
        })
    }

    fn is_pb(&self, split_time: Time) -> bool {
        self.split.map_or(false, |pb| split_time < pb)
    }
}
