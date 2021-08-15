//! Observation multiplexing.
use super::Observer;

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
    fn observe(&self, evt: super::Event) {
        // TODO(@MattWindsor91): eliminate redundant clone
        for o in &self.observers {
            o.observe(evt.clone())
        }
    }
}
