//! Split events in attempt observations.
use serde::{Deserialize, Serialize};

use crate::model;
use crate::model::timing;

/// Enumeration of split-level events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    // TODO(@MattWindsor91): remove this
    /// Initial dump of information about the event.
    Init { index: usize, name: String },
    /// Got a new time for the split.
    Time(model::Time, super::time::Event),
    /// Got a new pace note for the split.
    Pace(timing::comparison::pace::SplitInRun),
}

/// Trait for things that can observe split events.
pub trait Observer {
    /// Observes a split event `event` for split `split`.
    fn observe_split(&self, split: model::short::Name, event: Event);
}

/// Blanket implementation of time observers for split event observers.
impl<T: Observer> super::time::Observer for T {
    fn observe_time(
        &self,
        split: model::short::Name,
        time: model::Time,
        event: super::time::Event,
    ) {
        self.observe_split(split, Event::Time(time, event));
    }
}
