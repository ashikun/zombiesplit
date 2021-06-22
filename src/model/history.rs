/*!
Models related to a finished ('historic') run.

These models are useful for transferring run information into and out of
flat files, as well as storing finished runs into the database.
*/

use super::{short, time::Time};
use serde::{Deserialize, Serialize};

/// A summary of a finished run.
#[derive(Serialize, Deserialize)]
pub struct Run<D> {
    /// The category_locator used to locate the game and category.
    #[serde(flatten)]
    pub category_locator: D,
    /// Whether the run was completed.
    pub was_completed: bool,
    /// The date at which this run was archived.
    pub date: chrono::DateTime<chrono::Utc>,
    /// Map from split shortnames to times.
    pub times: short::Map<Time>,
}

impl<D> Run<D> {
    /// Creates a new run with the same contents as this one, but a new locator.
    pub fn with_locator<D2>(&self, category_locator: D2) -> Run<D2> {
        Run{
            category_locator,
            was_completed: self.was_completed,
            date: self.date,
            times: self.times.clone(),
        }
    }
}
