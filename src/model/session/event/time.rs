//! Observations for split times.
use crate::model::{short, timing};
use serde::{Deserialize, Serialize};

/// Enumeration of split time event types.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Time {
    /// This time was just added to the split.
    Pushed,
    /// This is a new aggregate time for this split.
    Aggregate(timing::aggregate::Kind),
}

/// Trait for things that can observe split time events.
pub trait Observer {
    /// Observes a split time event `event` for split `split`, with time `time`.
    fn observe_time(&self, split: short::Name, time: timing::Time, event: Time);

    /// Observes a set of aggregates `set` for split `split`, from source `source`.
    fn observe_aggregate_set(
        &self,
        split: short::Name,
        pair: timing::aggregate::Set,
        source: timing::aggregate::Source,
    ) {
        self.observe_aggregate(
            split,
            pair.split,
            source.with(timing::aggregate::Scope::Split),
        );
        self.observe_aggregate(
            split,
            pair.cumulative,
            source.with(timing::aggregate::Scope::Cumulative),
        );
    }

    /// Observes an aggregate time `time` for split `split`, of kind `kind`.
    fn observe_aggregate(
        &self,
        split: short::Name,
        time: timing::Time,
        kind: timing::aggregate::Kind,
    ) {
        self.observe_time(split, time, Time::Aggregate(kind));
    }
}
