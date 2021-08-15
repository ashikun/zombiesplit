//! Split events in attempt observations.
use crate::model;

use super::aggregate;

/// Enumeration of split-level events.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    Time(model::Time, Time),
    /// Got a new pace note for the split.
    Pace(model::comparison::pace::SplitInRun),
}

/// Enumeration of split-level time types.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Time {
    /// This time was just added to the split.
    Pushed,
    /// This time was just removed from the split.
    /// The UI may choose to load this time into a split editor.
    Popped,
    /// This is a new aggregate time for this split.
    Aggregate(aggregate::Kind),
}
