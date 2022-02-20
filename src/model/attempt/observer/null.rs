//! Null observers.

/// An observer that does nothing.
#[derive(Default)]
pub struct Null;

impl super::Observer for Null {
    fn observe(&self, _: super::Event) {}
}
