/*! A sink for attempted runs.

When a run is reset, the session may send it (completed or otherwise) to an
implementation of the [Sink] trait. */

use super::super::{game::category, history};

use thiserror::Error;

/// Type of runs accepted by a [Sink].
pub type Run = history::run::FullyTimed<category::ShortDescriptor>;

/// Trait for things that can accept a run.
pub trait Sink {
    /// Accepts the given run.
    ///
    /// # Errors
    ///
    /// Fails if the underlying storage mechanism (a database, for instance)
    /// can't store this run.
    fn accept(&mut self, run: Run) -> Result;
}

/// Shorthand for results from sinks.
pub type Result = std::result::Result<Outcome, Error>;

/// Possible outcomes of storing a run into a sink.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Outcome {
    /// The run was successfully saved.
    Saved,
    /// The run was ignored by the sink because it is not saving runs.
    Ignored,
}

/// Type of errors from sinks.
///
/// At time of writing, this just has one, catch-all error; eventually it may
/// expand to have more detailed error information.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Null implementation of a sink.
#[derive(Default)]
pub struct Null;

impl Sink for Null {
    fn accept(&mut self, _run: Run) -> Result {
        Ok(Outcome::Ignored)
    }
}
