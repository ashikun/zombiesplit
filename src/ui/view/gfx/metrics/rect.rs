//! The [Rect] struct and related functionality.

use super::conv::sat_i32;

/// Output-independent rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// Position of the top-left of this rectangle.
    pub top_left: super::Point,
    /// Size of the rectangle.
    pub size: super::Size,
}

impl Rect {
    /// Gets the right X co-ordinate.
    #[must_use]
    pub fn x2(self) -> i32 {
        self.top_left.x + self.size.w_i32()
    }

    /// Gets the bottom Y co-ordinate.
    #[must_use]
    pub fn y2(self) -> i32 {
        self.top_left.y + self.size.h_i32()
    }

    /// Produces a new [Rect] by shrinking the given [Rect] by `amount` on each side.
    #[must_use]
    pub fn shrink(self, amount: u32) -> Self {
        // TODO(@MattWindsor91): merge this and `grow`
        let amount_i32 = sat_i32(amount);
        Self {
            top_left: self.top_left.offset(amount_i32, amount_i32),
            size: self.size.shrink(amount * 2),
        }
    }

    /// Produces a new [Rect] by growing the given [Rect] by `amount` on each side.
    #[must_use]
    pub fn grow(self, amount: u32) -> Self {
        let amount_i32 = -sat_i32(amount);
        Self {
            top_left: self.top_left.offset(amount_i32, amount_i32),
            size: self.size.grow(amount * 2),
        }
    }
}
