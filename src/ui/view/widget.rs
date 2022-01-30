/*! Widgets.

The (reference) UI for zombiesplit contains several self-rendering widgets,
each of which has access to the presenter state and a renderer.
*/

use super::{super::presenter::State, gfx, layout};

mod footer;
mod header;
mod split;
mod time;

/// Trait for things that can render information from a presenter.
pub trait Widget<R: ?Sized>: super::layout::Layoutable {
    /// Type of state that this widget accepts.
    type State;

    /// Renders the widget.
    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()>;
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
        let mut bounds = ctx.bounds;
        for (w, h) in widget_heights(ctx) {
            bounds.size.h = h;
            self[w].layout(ctx.with_bounds(bounds));

            bounds.top_left.y += h;
        }
    }
}

impl<R: gfx::Renderer> Widget<R> for Root {
    type State = State;

    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()> {
        self.header.render(r, s)?;
        self.splits.render(r, s)?;
        self.footer.render(r, &s.footer)?;
        Ok(())
    }
}

/// Calculates heights for all of the widgets in the root widget.
fn widget_heights(ctx: layout::Context) -> Vec<(RootWidget, gfx::metrics::Length)> {
    // TODO(@MattWindsor91): probe each widget for its needed minimum height
    // TODO(@MattWindsor91): generalised layout algorithm?
    let header = ctx.config.window.header_h;
    let footer = ctx.config.window.footer_h;
    let splitset = ctx.bounds.size.h - header - footer;

    vec![
        (RootWidget::Header, header),
        (RootWidget::Splitset, splitset),
        (RootWidget::Footer, footer),
    ]
}

/// Enumeration of the various widgets stored on the root.
enum RootWidget {
    /// Represents the header widget.
    Header,
    /// Represents the splitset widget.
    Splitset,
    /// Represents the status widget.
    Footer,
}

impl std::ops::Index<RootWidget> for Root {
    type Output = dyn super::layout::Layoutable;

    fn index(&self, index: RootWidget) -> &Self::Output {
        match index {
            RootWidget::Header => &self.header,
            RootWidget::Splitset => &self.splits,
            RootWidget::Footer => &self.footer,
        }
    }
}

impl std::ops::IndexMut<RootWidget> for Root {
    fn index_mut(&mut self, index: RootWidget) -> &mut Self::Output {
        match index {
            RootWidget::Header => &mut self.header,
            RootWidget::Splitset => &mut self.splits,
            RootWidget::Footer => &mut self.footer,
        }
    }
}
