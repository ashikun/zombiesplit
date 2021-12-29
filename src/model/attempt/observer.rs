//! Observer pattern wiring for attempt sessions.

pub mod debug;
pub mod mux;
pub mod split;
pub mod time;

use super::super::{game::category, history, short};
use serde::{Deserialize, Serialize};

pub use debug::Debug;
pub use mux::Mux;

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
    /// Observes a full-run total (either from the comparison, or from the attempt).
    Total(
        crate::model::comparison::PacedTime,
        crate::model::aggregate::Source,
    ),
    /// Observes initial information about a split.
    AddSplit(short::Name, String),
    /// Observes a run reset, with any outgoing run attached as historic.
    Reset(Option<history::run::FullyTimed<category::ShortDescriptor>>),
    /// Observes information about the attempt number of a run.
    Attempt(category::AttemptInfo),
    /// Observes information about the game being run.
    GameCategory(category::Info),
    /// Observes an event on a split.
    Split(short::Name, split::Event),
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
    fn add_observer(&mut self, observer: std::rc::Weak<dyn Observer>);
}
