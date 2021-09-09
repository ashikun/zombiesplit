//! Observations for split times.
use crate::model;

/// Enumeration of split time event types.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// This time was just added to the split.
    Pushed,
    /// This time was just removed from the split.
    /// The UI may choose to load this time into a split editor.
    Popped,
    /// This is a new aggregate time for this split.
    Aggregate(model::aggregate::Kind),
}

/// Trait for things that can observe split time events.
pub trait Observer {
    /// Observes a split time event `event` for split `split`, with time `time`.
    fn observe_time(&self, split: model::short::Name, time: model::Time, event: Event);
}
