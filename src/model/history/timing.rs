/*! Models concerning time information for a historic run.

Because different parts of zombiesplit need to know different amounts of
information about the timing of a historic run, and each implies an
increasing amount of query complexity, there are several
different historic time models used.
*/
use super::super::{short, time::Time};
use serde::{Deserialize, Serialize};

pub trait Timing {
    /// Gets the total across all splits.
    fn total(&self) -> Time;
}

/// Full timing information for a run.
///
/// This includes every logged time for every split in the run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Full {
    pub times: short::Map<Vec<Time>>,
}

impl Timing for Full {
    fn total(&self) -> Time {
        self.times.values().cloned().flatten().sum()
    }
}

/// Split-total timing information for a run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Totals {
    pub totals: short::LinkedMap<Time>,
}

impl Timing for Totals {
    fn total(&self) -> Time {
        self.totals.values().copied().sum()
    }
}

/// Abbreviated timing information, usually returned from summary queries.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    /// The total time across all splits.
    pub total: Time,
    /// The rank of this run across all runs, if known.
    pub rank: Option<usize>,
}

impl Timing for Summary {
    fn total(&self) -> Time {
        self.total
    }
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

/// Dynamic choice of timing.
pub enum ForLevel {
    /// Wraps [Summary].
    Summary(Summary),
    /// Wraps [Totals].
    Totals(Totals),
    /// Wraps [Full].
    Full(Full),
}

impl From<Summary> for ForLevel {
    fn from(s: Summary) -> Self {
        Self::Summary(s)
    }
}

impl From<Totals> for ForLevel {
    fn from(s: Totals) -> Self {
        Self::Totals(s)
    }
}

impl From<Full> for ForLevel {
    fn from(s: Full) -> Self {
        Self::Full(s)
    }
}

impl Timing for ForLevel {
    fn total(&self) -> Time {
        match self {
            Self::Summary(f) => f.total(),
            Self::Totals(f) => f.total(),
            Self::Full(f) => f.total(),
        }
    }
}
