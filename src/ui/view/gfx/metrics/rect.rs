//! The [Rect] struct and related functionality.

use super::{
    anchor::{self, Anchor},
    conv::sat_i32,
    point::Point,
};

/// Output-independent rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    /// Position of the top-left of this rectangle.
    pub top_left: super::Point,
    /// Size of the rectangle.
    pub size: super::Size,
}

impl Rect {
    /// Resolves a point within a rectangle, given an offset (`dx`, `dy`) from
    /// `anchor`.
    #[must_use]
    pub fn point(self, dx: i32, dy: i32, anchor: Anchor) -> Point {
        Point {
            x: self.x(dx, anchor.x),
            y: self.y(dy, anchor.y),
        }
    }

    /// Resolves an X coordinate within a rectangle, given an offset `dx` from
    /// `anchor`.
    #[must_use]
    pub fn x(self, dx: i32, anchor: anchor::X) -> i32 {
        self.top_left.x
            + dx
            + match anchor {
                anchor::X::Left => 0,
                anchor::X::Right => self.size.w_i32(),
            }
    }

    /// Resolves an Y coordinate within a rectangle, given an offset `dy` from
    /// `anchor`.
    #[must_use]
    pub fn y(self, dy: i32, anchor: anchor::Y) -> i32 {
        self.top_left.y
            + dy
            + match anchor {
                anchor::Y::Top => 0,
                anchor::Y::Bottom => self.size.h_i32(),
            }
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