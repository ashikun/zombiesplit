//!  Models related to a finished ('historic') run.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A summary of a finished run.
///
/// Runs are parametric over category locators and time calculations.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

    /// Consumes this run and creates a new one with the same contents, but a
    /// new timing set derived by calling `f` on the existing one.
    pub fn map_timing<T2>(self, f: impl FnOnce(T) -> T2) -> Run<L, T2> {
        Run {
            category_locator: self.category_locator,
            was_completed: self.was_completed,
            date: self.date,
            timing: f(self.timing),
        }
    }

    /// Consumes this run and creates a new one with the same contents, but a
    /// new timing set.
    pub fn with_timing<T2>(self, timing: T2) -> Run<L, T2> {
        self.map_timing(|_| timing)
    }
}

/// A fully timed run.
pub type FullyTimed<L> = Run<L, super::timing::Full>;

/// A run with timing totals only.
pub type WithTotals<L> = Run<L, super::timing::Totals>;

/// A run with a summarised time only.
pub type Summary<L> = Run<L, super::timing::Summary>;

/// A run with a run-time-selected level of timing.
pub type ForLevel<L> = Run<L, super::timing::ForLevel>;
