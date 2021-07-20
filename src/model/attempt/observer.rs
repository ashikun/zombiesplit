//! Observer pattern wiring for attempt sessions.

use super::super::{game::category::ShortDescriptor, history};

/// An observer for the session.
pub trait Observer {
    /// Observes an event.
    ///
    /// The given session captures the state immediately before the
    /// reset.
    fn observe(&self, evt: Event);
}

/// Enumeration of events that can be sent through an observer
#[derive(Clone, Debug)]
pub enum Event {
    /// Observes a reset of a run, with any outgoing run attached as historic.
    Reset(Option<history::run::FullyTimed<ShortDescriptor>>),
}
