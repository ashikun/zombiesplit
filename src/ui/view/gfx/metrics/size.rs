//! The [Size] struct and related functionality.

use super::conv::{sat_i32, u32_or_zero};
use serde::{Deserialize, Serialize};

/// A two-dimensional size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Size {
    /// Width in pixels.
    pub w: u32,
    /// Height in pixels.
    pub h: u32,
}

impl Size {
    /// Shrinks the size in both dimensions by `amount`.
    #[must_use]
    pub fn shrink(self, amount: u32) -> Self {
        Self {
            w: self.w - amount,
            h: self.h - amount,
        }
    }

    /// Grows the size in both dimensions by `amount`.
    #[must_use]
    pub fn grow(self, amount: u32) -> Self {
        Self {
            w: self.w + amount,
            h: self.h + amount,
        }
    }

    /// Gets the width as a signed integer, saturating if needed.
    #[must_use]
    pub fn w_i32(self) -> i32 {
        sat_i32(self.w)
    }

    /// Gets the height as a signed integer, saturating if needed.
    #[must_use]
    pub fn h_i32(self) -> i32 {
        sat_i32(self.h)
    }

    /// Constructs a [Size] from a pair of signed width and height.
    /// Negative values are clipped to zero.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics;
    ///
    /// let size = metrics::Size::from_i32s(-5, 10);
    /// assert_eq!(0, size.w);
    /// assert_eq!(10, size.h);
    /// ```
    #[must_use]
    pub fn from_i32s(w: i32, h: i32) -> Self {
        Self {
            w: u32_or_zero(w),
            h: u32_or_zero(h),
        }
    }
}
