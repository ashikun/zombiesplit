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
#[derive(Default)]
pub struct Footer {
    /// The bounding box for the footer widget.
    rect: metrics::Rect,
    /// The rows configured into this Footer.
    rows: Vec<Row>,
}

impl layout::Layoutable for Footer {
    fn layout(&mut self, ctx: layout::Context) {
        self.rect = ctx.bounds;

        if self.rows.is_empty() {
            self.init_rows(ctx);
        }

        let w = self.rect.size.w;
        let mut top_left = self.rect.top_left;
        for row in &mut self.rows {
            let h = ctx.font_metrics[row.time.font_id].span_h(1);
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
    fn init_rows(&mut self, ctx: layout::Context) {
        self.rows = ctx
            .config
            .widgets
            .footer
            .rows
            .iter()
            .map(Row::new)
            .collect();
    }
}
