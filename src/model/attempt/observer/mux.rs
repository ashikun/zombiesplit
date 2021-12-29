//! Observation multiplexing.
use std::rc::Weak;

use super::{Observable, Observer};

/// An observation multiplexer.
///
/// A [Mux] contains zero or more weak references to [Observer]s.  It then
/// implements [Observer] itself, by distributing the incoming events to all of
/// the still-live attached observers.
#[derive(Default)]
pub struct Mux {
    observers: Vec<Weak<dyn Observer>>,
}

impl Observable for Mux {
    /// Adds an observer to the mux.
    fn add_observer(&mut self, observer: Weak<dyn Observer>) {
        self.observers.push(observer);
    }
}

impl Observer for Mux {
    fn observe(&self, evt: super::Event) {
        // TODO(@MattWindsor91): eliminate redundant clone
        for o in &self.observers {
            // TODO(@MattWindsor91): remove dead references
            if let Some(o) = o.upgrade() {
                o.observe(evt.clone());
            }
        }
    }
}
