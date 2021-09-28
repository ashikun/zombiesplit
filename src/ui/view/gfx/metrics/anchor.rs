//! Enumerations for anchoring a position to a rectangle.

/// A two-dimensional anchor.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    /// The X component of the anchor.
    pub x: X,
    /// The Y component of the anchor.
    pub y: Y,
}

impl Anchor {
    /// Top-left anchoring.
    pub const TOP_LEFT: Self = Anchor {
        x: X::Left,
        y: Y::Top,
    };

    /// Top-right anchoring.
    pub const TOP_RIGHT: Self = Anchor {
        x: X::Right,
        y: Y::Top,
    };

    /// Bottom-left anchoring.
    pub const BOTTOM_LEFT: Self = Anchor {
        x: X::Left,
        y: Y::Bottom,
    };

    /// Bottom-right anchoring.
    pub const BOTTOM_RIGHT: Self = Anchor {
        x: X::Right,
        y: Y::Bottom,
    };
}

/// An anchor for the X co-ordinate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum X {
    /// Anchoring to the left edge.
    Left,
    /// Anchoring to the right edge.
    Right,
}

impl X {}

/// An anchor for the Y co-ordinate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Y {
    /// Anchoring to the top edge.
    Top,
    /// anchoring to the bottom edge.
    Bottom,
}
