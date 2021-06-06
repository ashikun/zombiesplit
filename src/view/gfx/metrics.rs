//! Font and window metrics (most of which will be un-hardcoded later).
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

/// Window metrics.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Window {
    /// The window width.
    pub win_w: u32,
    /// The window height.
    pub win_h: u32,
    /// Standard padding on contents.
    pub padding: u32,
    /// The height of the header.
    pub header_h: u32,
    /// The height of the total section.
    pub total_h: u32,
    /// The height of one split.
    pub split_h: u32,
}

impl Window {
    /// Gets the bounding box of the header part of the window.
    #[must_use]
    pub fn header_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            size: Size {
                w: self.win_w,
                h: self.header_h,
            },
        }
        .shrink(self.padding)
    }

    /// Gets the bounding box of the splits part of the window.
    #[must_use]
    pub fn splits_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: self.splits_y(),
            size: Size {
                w: self.win_w,
                h: self.splits_h(),
            },
        }
        .shrink(self.padding)
    }

    /// Gets the bounding box of the total part of the window.
    #[must_use]
    pub fn total_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: self.total_y(),
            size: Size {
                w: self.win_w,
                h: self.total_h,
            },
        }
        .shrink(self.padding)
    }

    /// Gets the unshifted bounding box of the editor part of the window.
    #[must_use]
    pub fn editor_rect(&self) -> Rect {
        let mut r = self.splits_rect();
        r.size.h = self.split_h;
        r
    }

    /// Gets the Y position of the splits part of the window.
    fn splits_y(&self) -> i32 {
        sat_i32(self.header_h)
    }

    /// Gets the Y position of the total part of the window.
    fn total_y(&self) -> i32 {
        sat_i32(self.win_h - self.total_h)
    }

    /// Gets the height of the splits part of the window.
    fn splits_h(&self) -> u32 {
        self.win_h - self.header_h - self.total_h
    }
}

/// Convert `x` to i32, saturate if overly long.
pub(crate) fn sat_i32<T>(x: T) -> i32
where
    i32: TryFrom<T>,
{
    i32::try_from(x).unwrap_or(i32::MAX)
}

/// A two-dimensional size.
#[derive(Clone, Copy, Debug)]
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
}

/// Output-independent rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// X position of left.
    pub x: i32,
    /// Y position of top.
    pub y: i32,
    /// Size of the rectangle.
    pub size: Size,
}

impl Rect {
    /// Gets the right X co-ordinate.
    #[must_use]
    pub fn x2(self) -> i32 {
        self.x + self.size.w_i32()
    }

    /// Gets the bottom Y co-ordinate.
    #[must_use]
    pub fn y2(self) -> i32 {
        self.y + self.size.h_i32()
    }

    /// Produces a new [Rect] by shrinking the given [Rect] by `amount` on each side.
    #[must_use]
    pub fn shrink(self, amount: u32) -> Self {
        Self {
            x: self.x + sat_i32(amount),
            y: self.y + sat_i32(amount),
            size: self.size.shrink(amount * 2),
        }
    }

    /// Produces a new [Rect] by growing the given [Rect] by `amount` on each side.
    #[must_use]
    pub fn grow(self, amount: u32) -> Self {
        Self {
            x: self.x - sat_i32(amount),
            y: self.y - sat_i32(amount),
            size: self.size.grow(amount * 2),
        }
    }
}
