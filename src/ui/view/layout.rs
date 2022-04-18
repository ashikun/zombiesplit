//! Traits and objects relating to layout.

use ugly::{metrics, resource::Map};

use super::gfx::font;

/// Trait for things that can be laid out into the space defined by a context.
///
/// Layout is decoupled from rendering, and typically happens once at the start of the view creation
/// followed by occasional follow-ups if the size of the window changes.
pub trait Layoutable {
    /// Precalculate a minimal bounding box.
    ///
    /// The context `parent_ctx` is relative to the parent widget, and exists mainly to allow the
    /// widget to use font metrics to work out its size.
    fn min_bounds(&self, parent_ctx: Context) -> metrics::Size;

    /// Gets the actual bounding boc of this element, according to its last call to `layout`.
    fn actual_bounds(&self) -> metrics::Size;

    /// Calculates and stores a layout based on the context `ctx`.
    ///
    /// `ctx` is relative to the calculated bounding box of the widget,
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
    pub font_metrics: &'m font::Map<ugly::font::Metrics>,
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

    /// Applies the configured padding amount to `size`.
    #[must_use]
    pub fn pad_size(&self, size: metrics::Size) -> metrics::Size {
        size.grow(self.config.window.padding * 2)
    }

    /// Gets a copy of the font metrics for `font`.
    #[must_use]
    pub fn font_metrics(&self, font: super::gfx::font::Id) -> &'m ugly::font::Metrics {
        self.font_metrics.get(font)
    }
}
