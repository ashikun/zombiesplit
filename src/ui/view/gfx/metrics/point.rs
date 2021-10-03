//! The [Point] struct and related functionality.

use super::{Rect, Size};

/// A two-dimensional point.
///
/// Points can have negative coordinates, to allow relative offsetting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    /// The X-coordinate of the point.
    pub x: i32,
    /// The Y-coordinate of the point.
    pub y: i32,
}

impl Point {
    /// Offsets a [Point] by the given deltas, returning a new [Point].
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::point::Point;
    ///
    /// let p = Point { x: 0, y: 0 };
    /// let q = p.offset(0, 10);
    /// let r = q.offset(-4, 2);
    ///
    /// assert_eq!(0, p.x);
    /// assert_eq!(0, p.y);
    /// assert_eq!(0, q.x);
    /// assert_eq!(10, q.y);
    /// assert_eq!(-4, r.x);
    /// assert_eq!(12, r.y);
    /// ```
    #[must_use]
    pub fn offset(mut self, dx: i32, dy: i32) -> Self {
        self.offset_mut(dx, dy);
        self
    }

    /// Mutably offsets a point by the given deltas.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::point::Point;
    ///
    /// let mut p = Point { x: 0, y: 0 };
    /// p.offset_mut(0, 10);
    /// p.offset_mut(-4, 0);
    /// p.offset_mut(2, 2);
    ///
    /// assert_eq!(-2, p.x);
    /// assert_eq!(12, p.y);
    /// ```
    pub fn offset_mut(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Lifts this [Point] to a [Rect], with the point forming the top-left.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::{Point, Size};
    ///
    /// let p = Point { x: 4, y: 8 };
    /// let s = Size { w: 10, h: 2 };
    /// let r = p.to_rect(s);
    ///
    /// assert_eq!(p, r.top_left);
    /// assert_eq!(s, r.size);
    /// ```
    #[must_use]
    pub fn to_rect(self, size: Size) -> Rect {
        Rect {
            top_left: self,
            size,
        }
    }
}
