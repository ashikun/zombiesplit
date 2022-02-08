/*! Widgets.

The (reference) UI for zombiesplit contains several self-rendering widgets,
each of which has access to the presenter state and a renderer.
*/

use super::{
    super::presenter::State,
    gfx,
    layout::{self, Layoutable},
};

mod footer;
mod header;
pub mod label;
mod split;
pub mod stack;
mod time;

pub use label::Label;
pub use stack::Stack;

/// Trait for things that can render information from a presenter.
pub trait Widget<R: ?Sized>: super::layout::Layoutable {
    /// Type of state that this widget accepts.
    type State: ?Sized;

    /// Renders the widget.
    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()>;
}

/// The root widget.
///
/// Widgets
pub struct Root {
    /// The header widget.
    header: header::Widget,
    /// The splits widget.
    splits: split::Widget,
    /// The footer widget.
    footer: footer::Footer,
}

impl layout::Layoutable for Root {
    fn min_bounds(&self, _parent_ctx: layout::Context) -> gfx::metrics::Size {
        // This is never actually used, as there is no parent widget for the root.
        gfx::metrics::Size::default()
    }

    fn actual_bounds(&self) -> gfx::metrics::Size {
        // This is also never actually used.
        gfx::metrics::Size::default()
    }

    fn layout(&mut self, ctx: layout::Context) {
        let mut bounds = ctx.bounds;
        for (w, h) in self.widget_heights(ctx) {
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

impl Root {
    /// Constructs a new root widget using the given layout configuration.
    #[must_use]
    pub fn new(cfg: &super::config::layout::WidgetSet) -> Self {
        Self {
            header: header::Widget::default(),
            splits: split::Widget::default(),
            footer: footer::Footer::new(&cfg.footer),
        }
    }

    /// Calculates heights for all of the widgets in the root widget.
    fn widget_heights(&self, ctx: layout::Context) -> Vec<(RootWidget, gfx::metrics::Length)> {
        // TODO(@MattWindsor91): generalised layout algorithm?
        let header = self.header.min_bounds(ctx).h;
        let footer = self.footer.min_bounds(ctx).h;

        // We ignore the (empty) splitset min bounds and calculate it as the remainder of the
        // height after taking the other widgets into consideration.
        let splitset = ctx.bounds.size.h - header - footer;

        vec![
            (RootWidget::Header, header),
            (RootWidget::Splitset, splitset),
            (RootWidget::Footer, footer),
        ]
    }
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
