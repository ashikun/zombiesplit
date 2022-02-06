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
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::Size;
    ///
    /// assert_eq!(Size{w: 42, h: 22}, Size{w: 40, h:20}.grow(2));
    /// assert_eq!(Size{w: 40, h: 20}, Size{w: 42, h:22}.grow(-2));
    /// ```
    #[must_use]
    pub fn grow(self, amount: Length) -> Self {
        Self {
            w: self.w + amount,
            h: self.h + amount,
        }
    }

    /// Returns a size that is the maximum of `self` and `other` horizontally, and their sum
    /// vertically.
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::Size;
    ///
    /// assert_eq!(Size{w: 42, h: 32}, Size{w: 42, h:10}.stack_vertically(Size{w: 20, h: 22}));
    /// ```
    #[must_use]
    pub fn stack_vertically(self, other: Self) -> Self {
        Self {
            w: self.w.max(other.w),
            h: self.h + other.h,
        }
    }

    /// Returns a size that is the maximum of `self` and `other` vertically, and their sum
    /// horizontally.
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::Size;
    ///
    /// assert_eq!(Size{w: 62, h: 22}, Size{w: 42, h:10}.stack_horizontally(Size{w: 20, h: 22}));
    /// ```
    #[must_use]
    pub fn stack_horizontally(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            h: self.h.max(other.h),
        }
    }

    /// Merges multiple `sizes` into one using `f`, which should be one of `stack_horizontally` or
    /// `stack_vertically`.
    #[must_use]
    pub fn stack_many(
        sizes: impl IntoIterator<Item = Self>,
        f: impl Fn(Self, Self) -> Self,
    ) -> Self {
        sizes.into_iter().reduce(f).unwrap_or_default()
    }
}
