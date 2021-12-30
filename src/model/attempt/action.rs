/*!
An event interface for manipulating a current attempt.
*/

use crate::model::Time;
use serde::{Deserialize, Serialize};

/// An event that manipulates the current attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    async fn handle(&mut self, a: Action);
}
