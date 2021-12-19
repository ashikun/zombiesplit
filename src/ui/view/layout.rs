//! Traits and objects relating to layout.

use super::gfx::{font, metrics};

/// Trait for things that can be laid out into the space defined by a context.
///
/// Layout is decoupled from rendering, and typically happens once at the start of the view creation
/// followed by occasional follow-ups if the size of the window changes.
pub trait Layoutable {
    /// Asks the widget to calculate a layout based on the context `ctx`.
    fn layout(&mut self, ctx: Context);
}

/// Context used when performing a layout change.
#[derive(Debug, Clone, Copy)]
pub struct Context<'m> {
    /// The configured metrics for this split display window.
    ///
    /// Note that the window itself may not be the same size as the target
    /// size in these metrics, owing to possible resizing.
    pub wmetrics: metrics::Window,

    /// The bounding box of the widget itself.
    ///
    /// All widgets are placed and sized by their parents.
    pub bounds: metrics::Rect,

    /// A source of font metrics.
    ///
    /// This can be used for working out how large a piece of text might be.
    pub font_metrics: &'m font::Map<font::Metrics>,

    /// Information about which positions are enabled for time display.
    pub time_positions: &'m [Index],
}

impl<'m> Context<'m> {
    /// Makes a copy of this layout context with the given new bounding box.
    pub fn with_bounds(self, new_bounds: metrics::Rect) -> Self {
        Self {
            bounds: new_bounds,
            ..self
        }
    }
}

/// Layout information for one position index in a time layout.
///
/// A vector of these structures fully defines how the UI should render times.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Index {
    /// The index being displayed.
    pub index: crate::model::time::position::Index,
    /// The number of digits to display for this index.
    pub num_digits: u8,
}
