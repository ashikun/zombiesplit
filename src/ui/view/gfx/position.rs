//! Positioning setup for rendering.
use super::metrics::{sat_i32, Rect};

/// A position specification, used for moving the renderer's plotter.
///
/// [Position]s consist of an [X] component and a [Y] component, each of which
/// can be relative to one edge of the theoretical bounding rectangle, or
/// to the current position of the plotter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    /// The specification for the X coordinate.
    pub x: X,
    /// The specification for the Y coordinate.
    pub y: Y,
}

impl Position {
    /// Make a position relative to the top-left.
    #[must_use]
    pub fn top_left(x: i32, y: i32) -> Self {
        Self {
            x: X::Left(x),
            y: Y::Top(y),
        }
    }

    /// Make a position relative to the top-right.
    #[must_use]
    pub fn top_right(x: i32, y: i32) -> Self {
        Self {
            x: X::Right(x),
            y: Y::Top(y),
        }
    }

    /// Make a position relative to the current position.
    #[must_use]
    pub fn rel(dx: i32, dy: i32) -> Self {
        Self {
            x: X::Rel(dx),
            y: Y::Rel(dy),
        }
    }

    /// Make a position relative to the current position in terms of characters
    /// in a given font.
    #[must_use]
    pub fn rel_chars<T: super::font::metrics::TextSizer + ?Sized>(
        sizer: &T,
        dx: i32,
        dy: i32,
    ) -> Self {
        Self::rel(sizer.span_w(dx), sizer.span_h(dy))
    }

    /// Move X by `x` only.
    #[must_use]
    pub fn x(x: X) -> Self {
        Self { x, y: Y::Rel(0) }
    }

    /// Move Y by `y` only.
    #[must_use]
    pub fn y(y: Y) -> Self {
        Self { x: X::Rel(0), y }
    }

    /// Transforms coordinates relative to `rect` into coordinates that will
    /// apply to `rect`'s parent context.
    ///
    /// Any right/bottom relative coordinates will become top/left ones.
    #[must_use]
    pub fn normalise_to_rect(self, rect: Rect) -> Self {
        Self {
            x: self.x.normalise_to_rect(rect),
            y: self.y.normalise_to_rect(rect),
        }
    }
}

/// An X position specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X {
    /// Relative to current position.
    Rel(i32),
    /// Relative to the left side of the bounding rectangle.
    Left(i32),
    /// Relative to the right side of the bounding rectangle.
    Right(i32),
}

impl X {
    /// Transforms coordinates relative to `rect` into coordinates that will
    /// apply to `rect`'s parent context.
    ///
    /// Any right relative coordinates will become left ones.
    #[must_use]
    pub fn normalise_to_rect(self, rect: Rect) -> Self {
        match self {
            Self::Left(k) => Self::Left(rect.x + k),
            Self::Right(k) => Self::Left(rect.x2() - k),
            x @ Self::Rel(_) => x,
        }
    }

    /// Converts to a left position given the current X position and width.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::position::X;
    ///
    /// assert_eq!(27, X::Rel(2).to_left(25, 50));
    /// assert_eq!(10, X::Left(10).to_left(25, 50));
    /// assert_eq!(48, X::Right(2).to_left(25, 50));
    /// ```

    #[must_use]
    pub fn to_left(self, cur_x: i32, width: u32) -> i32 {
        match self {
            Self::Rel(dx) => cur_x + dx,
            Self::Left(x) => x,
            Self::Right(x) => sat_i32(width) - x,
        }
    }
}

/// A Y position specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Y {
    /// Relative to current position.
    Rel(i32),
    /// Relative to the top side of the bounding rectangle.
    Top(i32),
    /// Relative to the bottom side of the bounding rectangle.
    Bottom(i32),
}

impl Y {
    /// Transforms coordinates relative to `rect` into coordinates that will
    /// apply to `rect`'s parent context.
    ///
    /// Any bottom relative coordinates will become top ones.
    #[must_use]
    pub fn normalise_to_rect(self, rect: Rect) -> Self {
        match self {
            Self::Top(k) => Self::Top(rect.y + k),
            Self::Bottom(k) => Self::Top(rect.y2() - k),
            x @ Self::Rel(_) => x,
        }
    }

    /// Converts to a top position given the current Y position and height.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::position::Y;
    ///
    /// assert_eq!(27, Y::Rel(2).to_top(25, 50));
    /// assert_eq!(10, Y::Top(10).to_top(25, 50));
    /// assert_eq!(48, Y::Bottom(2).to_top(25, 50));
    /// ```
    #[must_use]
    pub fn to_top(self, cur_y: i32, height: u32) -> i32 {
        match self {
            Self::Rel(dy) => cur_y + dy,
            Self::Top(y) => y,
            Self::Bottom(y) => sat_i32(height) - y,
        }
    }
}
