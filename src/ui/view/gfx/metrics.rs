//! Font and window metrics.
use serde::{Deserialize, Serialize};

pub mod anchor;
pub mod point;
pub mod rect;
pub mod size;

pub use anchor::Anchor;
pub use point::Point;
pub use rect::Rect;
pub use size::{Length, Size};

/// Window metrics.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Window {
    /// The window width.
    pub win_w: Length,
    /// The window height.
    pub win_h: Length,
    /// Standard padding on contents.
    pub padding: Length,
    /// The height of the header.
    pub header_h: Length,
    /// The height of the total section.
    pub footer_h: Length,
    /// The height of one split.
    pub split_h: Length,
}

/// Default window metrics.
impl Default for Window {
    fn default() -> Self {
        Self {
            win_w: 320,
            win_h: 640,
            padding: 1,
            header_h: 40,
            footer_h: 64,
            split_h: 16,
        }
    }
}

impl Window {
    /// Gets the configured rectangle for the window.
    ///
    /// This determines the size that the window will take up initially; the
    /// window may be resized later on.
    #[must_use]
    pub fn win_rect(&self) -> Rect {
        Rect {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                w: self.win_w,
                h: self.win_h,
            },
        }
    }

    /// Gets the bounding box of the header part of the window.
    #[must_use]
    pub fn header_rect(&self) -> Rect {
        Rect {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                w: self.win_w,
                h: self.header_h,
            },
        }
        .grow(-self.padding)
    }

    /// Gets the bounding box of the splits part of the window.
    #[must_use]
    pub fn splits_rect(&self) -> Rect {
        Rect {
            top_left: Point {
                x: 0,
                y: self.header_h,
            },
            size: Size {
                w: self.win_w,
                h: self.splits_h(),
            },
        }
        .grow(-self.padding)
    }

    /// Gets the bounding box of the total part of the window.
    #[must_use]
    pub fn total_rect(&self) -> Rect {
        Rect {
            top_left: Point {
                x: 0,
                y: self.total_y(),
            },
            size: Size {
                w: self.win_w,
                h: self.footer_h,
            },
        }
        .grow(-self.padding)
    }

    /// Gets the Y position of the total part of the window.
    fn total_y(&self) -> Length {
        self.win_h - self.footer_h
    }

    /// Gets the height of the splits part of the window.
    fn splits_h(&self) -> Length {
        self.win_h - self.header_h - self.footer_h
    }
}
