//! Logic for drawing splits.

mod row;

use super::super::{
    super::presenter::state::State,
    gfx::{
        metrics::{Anchor, Length, Rect, Size},
        Renderer, Result,
    },
    layout::{self, Layoutable},
};

/// The split viewer widget.
#[derive(Default)]
pub struct Widget {
    /// The bounding box used for the widget.
    rect: Rect,
    /// The split drawer set, containing enough drawers for one layout.
    rows: Vec<row::Row>,
}

impl Layoutable for Widget {
    fn min_bounds(&self, _parent_ctx: layout::Context) -> Size {
        // Splitsets fill in any of the space remaining after the header/footer/etc, so there is
        // no minimum bounds.
        Size::default()
    }

    fn layout(&mut self, ctx: layout::Context) {
        let ctx = ctx.padded();
        self.rect = ctx.bounds;
        self.rows = rows(ctx);
    }
}

impl<R: Renderer> super::Widget<R> for Widget {
    type State = State;

    fn render(&self, r: &mut R, s: &Self::State) -> Result<()> {
        for (i, row) in self.rows.iter().enumerate() {
            // TODO(@MattWindsor91): calculate scroll point
            if let Some(split) = s.split_at_index(i) {
                row.render(r, split)?;
            }
        }
        Ok(())
    }
}

/// Constructs a vector of row widgets according to `ctx`.
fn rows(ctx: layout::Context) -> Vec<row::Row> {
    // Create a prototype row, measure it to see how tall it is, then use that to work out how
    // many we can fit in this layout.

    // TODO(@MattWindsor91): padding

    let row = row::Row::default();
    let split_h = row.min_bounds(ctx).h;

    let n_splits = ctx.bounds.size.h / split_h;
    (0..n_splits)
        .map(|n| {
            let mut r = row.clone();
            r.layout(ctx.with_bounds(row_bounds(ctx, split_h, n)));
            r
        })
        .collect()
}

fn row_bounds(ctx: layout::Context, split_h: Length, ix: Length) -> Rect {
    Rect {
        top_left: ctx.bounds.point(0, ix * split_h, Anchor::TOP_LEFT),
        size: Size {
            w: ctx.bounds.size.w,
            h: split_h,
        },
    }
}
