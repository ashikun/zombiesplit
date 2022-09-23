//! Split events in attempt observations.
use serde::{Deserialize, Serialize};

use crate::model::{
    short,
    timing::{comparison::delta, time::human},
};

/// Enumeration of split-level events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Split {
    /// Got a new time pushed to, or aggregated for, the split.
    Time(human::Time, super::Time),
    /// Got a new delta for the split.
    Delta(delta::Split),
    /// One or more times have been popped from the split.
    Popped(super::super::action::Pop),
}

/// Trait for things that can observe split events.
pub trait Observer {
    /// Observes a split event `event` for split `split`.
    fn observe_split(&self, split: short::Name, event: Split);
}

/// Blanket implementation of time observers for split event observers.
impl<T: Observer> super::time::Observer for T {
    fn observe_time(&self, split: short::Name, time: human::Time, event: super::time::Time) {
        self.observe_split(split, Split::Time(time, event));
    }
}
