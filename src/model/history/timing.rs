/*! Models concerning time information for a historic run.

Because different parts of zombiesplit need to know different amounts of
information about the timing of a historic run, and each implies an
increasing amount of query complexity, there are several
different historic time models used.
*/
use super::super::{short, time::Time};
use serde::{Deserialize, Serialize};

/// Full timing information for a run.
///
/// This includes every logged time for every split in the run.
#[derive(Clone, Serialize, Deserialize)]
pub struct Full {
    pub times: short::Map<Vec<Time>>,
}

/// Split-total timing information for a run.
#[derive(Clone, Serialize, Deserialize)]
pub struct Totals {
    pub totals: short::Map<Time>,
}

/// Abbreviated timing information, usually returned from summary queries.
#[derive(Clone, Serialize, Deserialize)]
pub struct Summary {
    /// The total time across all splits.
    pub total: Time,
    /// The rank of this run across all runs, if known.
    pub rank: Option<usize>,
}

/// Enumeration of the various timing levels.
///
/// This is useful for presenting a choice of which timing level to get.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Represents [Summary].
    Summary,
    /// Represents [Totals].
    Totals,
    /// Represents [Full].
    Full,
}
