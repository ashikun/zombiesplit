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
    /// The user layout configuration.
    pub config: &'m super::config::Layout,

    /// The bounding box of the widget itself.
    ///
    /// All widgets are placed and sized by their parents.
    pub bounds: metrics::Rect,

    /// A source of font metrics.
    ///
    /// This can be used for working out how large a piece of text might be.
    pub font_metrics: &'m font::Map<font::Metrics>,
}

impl<'m> Context<'m> {
    /// Makes a copy of this layout context with the given new bounding box.
    pub fn with_bounds(self, new_bounds: metrics::Rect) -> Self {
        Self {
            bounds: new_bounds,
            ..self
        }
    }

    /// Makes a copy of this layout context with the bounding box shrunk by the padding amount.
    pub fn padded(self) -> Self {
        self.with_bounds(self.bounds.grow(-self.config.window.padding))
    }
}
