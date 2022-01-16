/*! Indexing enumerations for aggregates.

These are useful for parametrically varying the specific [Source] or [Scope]
in an aggregate structure.
*/

use serde::{Deserialize, Serialize};

/// The kind ([Source] and [Scope]) of an aggregate time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Kind {
    /// The source of this aggregate time.
    pub source: Source,
    /// The scope of this aggregate time.
    pub scope: Scope,
}

impl Kind {
    /// A cumulative time from the current attempt.
    pub const ATTEMPT_CUMULATIVE: Self = Source::Attempt.with(Scope::Cumulative);

    /// A split time from the current attempt.
    pub const ATTEMPT_SPLIT: Self = Source::Attempt.with(Scope::Split);

    /// A cumulative time from the comparison.
    pub const COMPARISON_CUMULATIVE: Self = Source::Comparison.with(Scope::Cumulative);

    /// A split time from the comparison.
    pub const COMPARISON_SPLIT: Self = Source::Comparison.with(Scope::Split);
}

/// Enumeration of sources for aggregate times.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Source {
    /// This time comes from the run attempt.
    Attempt,
    /// This time comes from the comparison; ie, it is the time to which we are
    /// comparing.
    Comparison,
}

impl Source {
    /// Creates a `Kind` using this `Source` and a given `Scope` `scope`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zombiesplit::model::timing::aggregate::{Source, Scope, Kind};
    ///
    /// let x = Source::Attempt.with(Scope::Cumulative);
    /// assert_eq!(Source::Attempt, x.source);
    /// assert_eq!(Scope::Cumulative, x.scope);
    /// assert_eq!(Kind::ATTEMPT_CUMULATIVE, x);

    /// ```
    #[must_use]
    pub const fn with(self, scope: Scope) -> Kind {
        Kind {
            source: self,
            scope,
        }
    }
}

/// Enumeration of scopes for aggregate times.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Scope {
    /// Total of all times logged on the split.
    Split,
    /// Cumulative run time at this split; ie, the total plus
    /// all totals of all preceding runs.
    Cumulative,
}
