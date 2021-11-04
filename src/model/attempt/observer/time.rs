//! Observations for split times.
use crate::model::{self, aggregate, short};

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
    fn observe_time(&self, split: short::Name, time: model::Time, event: Event);

    /// Observes a set of aggregates `set` for split `split`, from source `source`.
    fn observe_aggregate_set(
        &self,
        split: short::Name,
        pair: aggregate::Set,
        source: aggregate::Source,
    ) {
        self.observe_aggregate(split, pair.split, source.with(aggregate::Scope::Split));
        self.observe_aggregate(
            split,
            pair.cumulative,
            source.with(aggregate::Scope::Cumulative),
        );
    }

    /// Observes an aggregate time `time` for split `split`, of kind `kind`.
    fn observe_aggregate(&self, split: short::Name, time: model::Time, kind: aggregate::Kind) {
        self.observe_time(split, time, Event::Aggregate(kind));
    }
}
