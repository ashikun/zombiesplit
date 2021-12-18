/*! Widgets.

The (reference) UI for zombiesplit contains several self-rendering widgets,
each of which has access to the presenter state and a renderer.
*/

use super::{
    super::presenter::State,
    gfx::{self, font, metrics, render},
};

mod footer;
mod header;
mod split;
mod time;

/// Trait for things that can render information from a presenter.
pub trait Widget<State> {
    /// Asks the widget to calculate a layout based on the context `ctx`.
    fn layout(&mut self, ctx: LayoutContext);

    /// Renders the widget (excluding its children).
    ///
    /// By default, implementations do nothing here.
    fn render(&self, _r: &mut dyn render::Renderer, _s: &State) -> gfx::Result<()> {
        Ok(())
    }

    /// Gets all immediate children of this widget.
    ///
    /// By default, widgets have no children.
    fn children(&self) -> Vec<&dyn Widget<State>> {
        vec![]
    }
}

/// Context used when performing a layout change.
#[derive(Clone, Copy)]
pub struct LayoutContext<'m> {
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
    pub time_positions: &'m [IndexLayout],
}

impl<'m> LayoutContext<'m> {
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
pub struct IndexLayout {
    /// The index being displayed.
    pub index: crate::model::time::position::Index,
    /// The number of digits to display for this index.
    pub num_digits: u8,
}

/// The root widget.
///
/// Widgets
#[derive(Default)]
pub struct Root {
    /// The header widget.
    header: header::Widget,
    /// The splits widget.
    splits: split::Widget,
    /// The footer widget.
    footer: footer::Footer,
}

impl Widget<State> for Root {
    fn layout(&mut self, ctx: LayoutContext) {
        self.header
            .layout(ctx.with_bounds(ctx.wmetrics.header_rect()));
        self.splits
            .layout(ctx.with_bounds(ctx.wmetrics.splits_rect()));
        self.footer
            .layout(ctx.with_bounds(ctx.wmetrics.total_rect()));
    }

    fn children(&self) -> Vec<&dyn Widget<State>> {
        vec![&self.header, &self.splits, &self.footer]
    }
}
