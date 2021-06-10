//! Models relating to an in-progress run.

use super::{
    split::{Set, Split},
    time::Time,
};

/// An in-progress run.
pub struct Run {
    /// The attempt number of this run.
    pub attempt: usize,
    /// The split data for this run.
    pub splits: Vec<Split>,
}

/// A run is a set of splits.
impl Set for Run {
    fn reset(&mut self) {
        self.attempt += 1;
        self.splits.iter_mut().for_each(Split::clear)
    }

    fn push_to(&mut self, split: usize, time: Time) {
        if let Some(ref mut s) = self.splits.get_mut(split) {
            s.push(time)
        }
    }

    fn pop_from(&mut self, split: usize) -> Option<Time> {
        self.splits.get_mut(split).and_then(Split::pop)
    }

    fn clear_at(&mut self, split: usize) {
        if let Some(s) = self.splits.get_mut(split) {
            s.clear()
        }
    }

    fn total_at(&self, split: usize) -> Time {
        self.splits
            .iter()
            .take(split + 1)
            .map(Split::summed_time)
            .sum()
    }

    fn time_at(&self, split: usize) -> Time {
        self.splits
            .get(split)
            .map_or(Time::default(), Split::summed_time)
    }

    fn num_times_at(&self, split: usize) -> usize {
        self.splits.get(split).map_or(0, Split::num_times)
    }

    fn num_splits(&self) -> usize {
        self.splits.len()
    }

    fn name_at(&self, split: usize) -> &str {
        self.splits.get(split).map_or("Unknown", |s| &s.name)
    }
}
