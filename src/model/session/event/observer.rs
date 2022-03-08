//! Event observers and observables.
use crate::model::session::event::{split, Event};

/// An observer for the session.
pub trait Observer {
    /// Observes an event.
    ///
    /// The given session captures the state immediately before the
    /// reset.
    fn observe(&self, evt: Event);
}

/// Blanket implementation for split observing on model observers.
impl<T: Observer> split::Observer for T {
    fn observe_split(&self, split: crate::model::short::Name, event: split::Split) {
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

/// An observer that does nothing.
#[derive(Default)]
pub struct Null;

impl Observer for Null {
    fn observe(&self, _: super::Event) {}
}
