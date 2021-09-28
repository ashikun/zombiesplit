//! The [Point] struct and related functionality.

/// A two-dimensional point.
///
/// Points can have negative coordinates, to allow relative offsetting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
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
}
