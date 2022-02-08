//! Font and window metrics.
use serde::{Deserialize, Serialize};

pub mod anchor;
pub mod axis;
pub mod point;
pub mod rect;
pub mod size;

pub use anchor::Anchor;
pub use axis::Axis;
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
}

/// Default window metrics.
impl Default for Window {
    fn default() -> Self {
        Self {
            win_w: 320,
            win_h: 640,
            padding: 1,
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
