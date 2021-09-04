//! Observer pattern wiring for attempt sessions.

pub mod mux;
pub mod split;

use crate::model::short;

use super::super::{game::category, history};

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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
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
