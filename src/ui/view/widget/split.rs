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
    fn layout(&mut self, ctx: layout::Context) {
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
    // TODO(@MattWindsor91): padding
    let split_h = ctx.config.window.split_h;
    let n_splits = usize::try_from(ctx.bounds.size.h / split_h).unwrap_or_default();
    (0..n_splits).map(|n| row(ctx, n)).collect()
}

fn row(ctx: layout::Context, index: usize) -> row::Row {
    let mut r = row::Row::default();
    r.layout(ctx.with_bounds(row_bounds(ctx, index)));
    r
}

fn row_bounds(ctx: layout::Context, index: usize) -> Rect {
    let split_h = ctx.config.window.split_h;
    let ix: Length = index.try_into().unwrap_or_default();
    Rect {
        top_left: ctx.bounds.point(0, ix * split_h, Anchor::TOP_LEFT),
        size: Size {
            w: ctx.bounds.size.w,
            h: split_h,
        },
    }
}
