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
    /// To shrink, grow by a negative amount.  Neither dimension will shrink past 0.
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
            w: (self.w + amount).max(0),
            h: (self.h + amount).max(0),
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

    /// Gets whether either dimension of this size is zero.
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::Size;
    ///
    /// assert!(Size{w: 0, h: 10}.is_zero());
    /// assert!(Size{w: 10, h: 0}.is_zero());
    /// assert!(!Size{w: 10, h: 10}.is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.h <= 0 || self.w <= 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Sizes must not be made negative by growing them by negative amounts.
    #[test]
    fn grow_negative_clamp() {
        assert_eq!(Size::default(), Size::default().grow(-1));
    }
}
