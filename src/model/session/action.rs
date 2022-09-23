/*!
An event interface for manipulating a current attempt.
*/

use crate::model::timing::time::human;
use serde::{Deserialize, Serialize};

/// An event that manipulates the current session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Action {
    /// Start a new run.
    NewRun(OldDestination),
    /// Pushes a time to the split at the given position.
    Push(usize, human::Time),
    /// Pops one or more times from the split at the given position.
    Pop(usize, Pop),
}

/// What should we do with an old attempt when we start a new one?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OldDestination {
    /// Save the old attempt.
    Save,
    /// Discard the old attempt.
    Discard,
}

/// Type of pop used in pop actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Pop {
    /// Pop one time.
    One,
    /// Pop all times.
    All,
}

/// Trait of things that perform actions.
///
/// These can be sessions, mocks, inter-process communications, or something else.
pub trait Handler {
    /// Type of errors returned by the handler.
    type Error: std::error::Error;

    /// Asks the handler to dump its current state.
    ///
    /// Dumping has to be mutable to accomodate situations such as `gRPC` where there is no notion
    /// of procedure calls being immutable.
    ///
    /// # Errors
    ///
    /// Fails if we can't, for whatever reason, get a dump from the handler.
    fn dump(&mut self) -> Result<super::State, Self::Error>;

    // TODO(@MattWindsor91): errors?

    /// Performs the action `a`.
    ///
    /// # Errors
    ///
    /// Fails if we can't, for whatever reason, perform the action.
    fn handle(&mut self, a: Action) -> Result<(), Self::Error>;
}
