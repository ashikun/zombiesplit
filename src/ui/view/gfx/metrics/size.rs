//! The [Size] struct and related functionality.

use serde::{Deserialize, Serialize};

/// Type of pixel lengths (and deltas on lengths).
///
/// We keep both lengths and deltas signed to avoid needing to do a lot of type conversion.
pub type Length = i32;

/// A two-dimensional size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Size {
    /// Width in pixels.
    pub w: Length,
    /// Height in pixels.
    pub h: Length,
}

impl Size {
    /// Grows the size in both dimensions by `amount`.
    ///
    /// To shrink, grow by a negative amount.
    #[must_use]
    pub fn grow(self, amount: Length) -> Self {
        Self {
            w: self.w + amount,
            h: self.h + amount,
        }
    }
}
