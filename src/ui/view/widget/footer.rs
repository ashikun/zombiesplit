//! The split total widget.

use ugly::metrics;

use row::Row;

use super::super::{
    super::presenter::state,
    gfx::Renderer,
    layout::{self, Layoutable},
    update::{self, Updatable},
};

mod row;

/// The footer widget.
pub struct Footer {
    /// The outer bounding box for the footer widget.
    bounds: metrics::Rect,

    /// The padded inner box for the footer widget.
    rect: metrics::Rect,

    /// The rows configured into this Footer.
    rows: super::stack::Stack<Row>,
}

impl Layoutable for Footer {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        self.rows
            .min_bounds(parent_ctx)
            .grow(2 * parent_ctx.config.window.padding)
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        let ctx = ctx.padded();
        self.rect = ctx.bounds;

        self.rows.layout(ctx);
    }
}

impl Updatable for Footer {
    type State = state::Footer;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        self.rows.update(ctx, s);
    }
}

impl<'r, R: Renderer<'r>> super::Widget<R> for Footer {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.rows.render(r)
    }
}

impl Footer {
    /// Constructs a new footer widget.
    #[must_use]
    pub fn new(cfg: &super::super::config::layout::Footer) -> Self {
        let mut rows = super::Stack::new(metrics::Axis::Vertical);
        rows.extend(cfg.rows.iter().map(|x| (Row::new(x), 0)));

        Self {
            bounds: metrics::Rect::default(),
            rect: metrics::Rect::default(),
            rows,
        }
    }
}
