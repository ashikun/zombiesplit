//! The split total widget.

use row::Row;

use super::{
    super::{
        super::presenter::state,
        gfx::{
            self,
            metrics::{self, Anchor, Size},
            Renderer,
        },
    },
    layout,
};

mod row;

/// The footer widget.
pub struct Footer {
    /// The bounding box for the footer widget.
    rect: metrics::Rect,
    /// The rows configured into this Footer.
    rows: Vec<Row>,
}

impl layout::Layoutable for Footer {
    fn min_bounds(&self, parent_ctx: layout::Context) -> Size {
        Size::stack_many(
            self.rows
                .iter()
                .map(|x| layout::Layoutable::min_bounds(x, parent_ctx)),
            Size::stack_vertically,
        )
        .grow(2 * parent_ctx.config.window.padding)
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.rect = ctx.padded().bounds;

        let w = self.rect.size.w;
        let mut top_left = self.rect.top_left;
        for row in &mut self.rows {
            let h = row.min_bounds(ctx).h;
            let row_rect = top_left.to_rect(Size { w, h }, Anchor::TOP_LEFT);
            row.layout(ctx.with_bounds(row_rect));
            top_left.offset_mut(0, h);
        }
    }
}

impl<R: Renderer> super::Widget<R> for Footer {
    type State = state::Footer;

    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()> {
        for row in &self.rows {
            row.render(r, s)?;
        }
        Ok(())
    }
}

impl Footer {
    /// Constructs a new footer widget.
    #[must_use]
    pub fn new(cfg: &super::super::config::layout::Footer) -> Self {
        Self {
            rect: Default::default(),
            rows: cfg.rows.iter().map(Row::new).collect(),
        }
    }
}
