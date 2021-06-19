/*!
Models related to a finished run.

These models are useful for transferring run information into and out of
flat files, as well as storing finished runs into the database.
*/

use super::{short, time::Time};
use serde::{Deserialize, Serialize};

/// A summary of a finished run.
#[derive(Serialize, Deserialize)]
pub struct Summary<D> {
    /// The descriptor used to locate the game and category.
    #[serde(flatten)]
    pub descriptor: D,
    /// Whether the run was completed.
    pub was_completed: bool,
    /// The date at which this run was archived.
    pub date: chrono::DateTime<chrono::Utc>,
    /// Map from split shortnames to times.
    pub times: short::Map<Time>,
}

impl Summary<super::game::category::ShortDescriptor> {}
