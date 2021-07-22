//! Observer pattern wiring for attempt sessions.

use super::super::{game::category, history};

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
    /// Observes a run reset, with any outgoing run attached as historic.
    Reset(Option<history::run::FullyTimed<category::ShortDescriptor>>),
    /// Observes information about the attempt number of a run.
    Attempt(category::AttemptInfo),
    /// Observes information about the game being run.
    GameCategory(category::Info),
}

/// An observation multiplexer.
#[derive(Default)]
pub struct Mux<'a> {
    observers: Vec<Box<dyn Observer + 'a>>,
}

impl<'a> Mux<'a> {
    /// Adds an observer to the mux.
    pub fn add(&mut self, obs: Box<dyn Observer + 'a>) {
        self.observers.push(obs)
    }
}

impl<'a> Observer for Mux<'a> {
    fn observe(&self, evt: Event) {
        // TODO(@MattWindsor91): eliminate redundant clone
        for o in &self.observers {
            o.observe(evt.clone())
        }
    }
}
