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
    // TODO(@MattWindsor91): this should eventually be reduced to just win_w/h and padding.
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
    /// Gets the configured size for the window.
    ///
    /// This determines the size that the window will take up initially; the
    /// window may be resized later on.
    #[must_use]
    pub fn win_size(&self) -> Size {
        Size {
            w: self.win_w,
            h: self.win_h,
        }
    }
}
