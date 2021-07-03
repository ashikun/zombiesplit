//! Observer pattern wiring for attempt sessions.

use super::session::Session;

/// An observer for the session.
pub trait Observer {
    /// Observes a reset.
    ///
    /// The given session captures the state immediately before the
    /// reset.
    fn on_reset(&self, _session: &Session) {}
}
