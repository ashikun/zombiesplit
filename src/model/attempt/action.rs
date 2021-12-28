/*!
An event interface for manipulating a current attempt.
*/

use crate::model::Time;

/// An event that manipulates the current attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Action {
    /// Dump state to all observers.
    Dump,
    /// Start a new run.
    NewRun,
    /// Pushes a time to the split at the given position.
    Push(usize, Time),
    /// Pops a time from the split at the given position.
    Pop(usize),
    /// Erases the split at the given position.
    Clear(usize),
}

/// Trait of things that perform actions.
///
/// These can be sessions, mocks, inter-process communications, or something else.
pub trait Handler {
    // TODO(@MattWindsor91): errors?

    /// Performs the action `a`.
    fn handle(&mut self, a: Action);

    /// Adds an observer, so that the effect of an action can be seen.
    fn add_observer(&mut self, observer: std::rc::Weak<dyn super::Observer>);
}
