/*!
An event interface for manipulating a current attempt through a [Session].
*/

use crate::model::Time;

/// An event that manipulates the current attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Action {
    /// Start a new run.
    NewRun,
    /// Pushes a time to the split at the given position.
    Push(usize, Time),
    /// Pops a time from the split at the given position.
    Pop(usize),
    /// Erases the split at the given position.
    Clear(usize),
}
