/*!
Models related to a finished ('historic') run.

These models are useful for transferring run information into and out of
flat files, as well as storing finished runs into the database.
*/

use super::{short, time::Time};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A summary of a finished run.
///
/// Runs are parametric over category locators and time calculations.
#[derive(Serialize, Deserialize)]
pub struct Run<L, T> {
    /// The category_locator used to locate the game and category.
    #[serde(flatten)]
    pub category_locator: L,
    /// Timing information for the run.
    #[serde(flatten)]
    pub timing: T,
    /// Whether the run was completed.
    pub was_completed: bool,
    /// The date at which this run was archived.
    pub date: DateTime<Utc>,
}

impl<L, T: Clone> Run<L, T> {
    /// Creates a new run with the same contents as this one, but a new locator.
    pub fn with_locator<L2>(&self, category_locator: L2) -> Run<L2, T> {
        Run {
            category_locator,
            was_completed: self.was_completed,
            date: self.date,
            timing: self.timing.clone(),
        }
    }
}

/// Full timing information for a run.
#[derive(Clone, Serialize, Deserialize)]
pub struct FullTiming {
    pub times: short::Map<Vec<Time>>,
}

/// Abbreviated timing information, usually returned from summary queries.
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeSummary {
    /// The total time across all splits.
    pub total: Time,
    /// The rank of this run across all runs.
    pub rank: Option<usize>,
}

/// A fully timed run.
pub type TimedRun<L> = Run<L, FullTiming>;

/// A run with a summarised time only.
pub type RunSummary<L> = Run<L, TimeSummary>;
