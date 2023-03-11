//! Run timing constructs.
use super::super::time;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

/// A set of calculated totals for a run.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Run {
    /// The total time inside the PB run, if any.
    pub total_in_pb_run: Option<time::Time>,
    /// The sum of best segment times in the comparison.
    pub sum_of_best: Option<time::Time>,
}

impl Run {
    /// Iterates over all of the totals stored in this [Run].
    pub fn totals(&'_ self) -> impl Iterator<Item = (TotalType, Option<time::Time>)> + '_ {
        vec![TotalType::TotalInPbRun, TotalType::SumOfBest]
            .into_iter()
            .map(|x| (x, *self.index(x)))
    }
}

/// Enumeration of types of run-wide total seen in a comparison.
///
/// These can be used to index a [Run].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TotalType {
    /// Denotes the total time inside the PB run.
    TotalInPbRun,
    /// Denotes the sum-of-best.
    SumOfBest,
}

impl Index<TotalType> for Run {
    type Output = Option<time::Time>;

    fn index(&self, index: TotalType) -> &Self::Output {
        match index {
            TotalType::TotalInPbRun => &self.total_in_pb_run,
            TotalType::SumOfBest => &self.sum_of_best,
        }
    }
}

impl IndexMut<TotalType> for Run {
    fn index_mut(&mut self, index: TotalType) -> &mut Self::Output {
        match index {
            TotalType::TotalInPbRun => &mut self.total_in_pb_run,
            TotalType::SumOfBest => &mut self.sum_of_best,
        }
    }
}
