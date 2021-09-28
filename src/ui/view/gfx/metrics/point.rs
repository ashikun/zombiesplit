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
    /// Offsets a point by the given deltas.
    #[must_use]
    pub fn offset(mut self, dx: i32, dy: i32) -> Self {
        self.x += dx;
        self.y += dy;
        self
    }
}
