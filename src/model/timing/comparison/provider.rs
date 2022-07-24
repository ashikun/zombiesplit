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

/// Comparisons, themselves, are comparison providers.
///
/// More specifically, anything that can be turned into an optional comparison is a comparison
/// provider.
impl<C: Into<Option<Comparison>> + Clone> Provider for C {
    fn comparison(&mut self) -> Result {
        Ok(self.clone().into())
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
