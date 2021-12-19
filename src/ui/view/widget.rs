/*! Widgets.

The (reference) UI for zombiesplit contains several self-rendering widgets,
each of which has access to the presenter state and a renderer.
*/

use super::{
    super::presenter::State,
    gfx::{self, metrics, render},
    layout,
};
use crate::ui::view::gfx::Renderer;

mod footer;
mod header;
mod split;
mod time;

/// Trait for things that can render information from a presenter.
pub trait Widget: super::layout::Layoutable {
    /// Type of state that this widget accepts.
    type State;

    /// Renders the widget.
    fn render(&self, r: &mut dyn render::Renderer, s: &Self::State) -> gfx::Result<()>;
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

impl layout::Layoutable for Root {
    fn layout(&mut self, ctx: layout::Context) {
        self.header
            .layout(ctx.with_bounds(ctx.wmetrics.header_rect()));
        self.splits
            .layout(ctx.with_bounds(ctx.wmetrics.splits_rect()));
        self.footer
            .layout(ctx.with_bounds(ctx.wmetrics.total_rect()));
    }
}

impl Widget for Root {
    type State = State;

    fn render(&self, r: &mut dyn Renderer, s: &Self::State) -> gfx::Result<()> {
        self.header.render(r, s)?;
        self.splits.render(r, s)?;
        self.footer.render(r, &s.footer)?;
        Ok(())
    }
}
