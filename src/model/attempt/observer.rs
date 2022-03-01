//! Observer pattern wiring for attempt sessions.

pub mod debug;
pub mod mux;
mod null;
pub mod split;
pub mod time;

use super::super::{game::category, short, timing};
use serde::{Deserialize, Serialize};

pub use debug::Debug;
pub use mux::Mux;
pub use null::Null;

/// An observer for the session.
pub trait Observer {
    /// Observes an event.
    ///
    /// The given session captures the state immediately before the
    /// reset.
    fn observe(&self, evt: Event);
}

/// Enumeration of events that can be sent through an observer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    /// Observes a change in one of the run total times.
    Total(Total),
    /// Observes the number of splits, which can be used to prepare for incoming split information.
    ///
    /// This SHOULD be sent before `Split` events give `Init` information about the splits.
    NumSplits(usize),
    /// Observes information about a reset, with the new attempt information attached.
    Reset(category::AttemptInfo),
    /// Observes information about the game being run.
    GameCategory(category::Info),
    /// Observes an event on a split.
    Split(short::Name, split::Event),
}

/// A run-total time event.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Total {
    /// Total so far in the attempt.
    Attempt(timing::comparison::PacedTime),
    /// The comparison time.
    Comparison(timing::Time),
    /// The sum of best segment times in the comparison.
    SumOfBest(timing::Time),
}

/// Blanket implementation for split observing on model observers.
impl<T: Observer> split::Observer for T {
    fn observe_split(&self, split: crate::model::short::Name, event: split::Event) {
        self.observe(Event::Split(split, event));
    }
}

/// Trait for things that can be observed.
///
/// These are usually multiplexers, sessions, or some sort of proxy for one of those two.
pub trait Observable {
    /// Adds an observer, so that the effect of an action can be seen.
    ///
    /// Observers are atomic weak references; this is because
    fn add_observer(&mut self, observer: std::sync::Weak<dyn Observer>);
}
