/// Comparison providers.
use super::Comparison;

/// Trait of objects that can provide comparisons.
pub trait Provider {
    /// Gets the current comparison for a game-category.
    ///
    /// # Errors
    ///
    /// Propagates forwards any errors from the particular comparison method itself.
    fn comparison(&mut self) -> Result;
}

/// A provider that never provides comparisons.
pub struct Null;

impl Provider for Null {
    fn comparison(&mut self) -> Result {
        Ok(None)
    }
}

/// Enumeration of errors that can occur when getting a comparison.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An arbitrary error has occurred while getting the comparison.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Shorthand for results from comparison providers.
pub type Result = std::result::Result<Option<Comparison>, Error>;
