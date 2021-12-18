//! Logic for drawing splits.

mod editor;
mod row;

use super::{
    super::{
        super::{presenter::state, view::widget},
        gfx::metrics::{conv::sat_i32, Anchor, Rect, Size},
    },
    LayoutContext,
};

/// The split viewer widget.
#[derive(Default)]
pub struct Widget {
    /// The bounding box used for the widget.
    rect: Rect,
    /// The split drawer set, containing enough drawers for one layout.
    splits: Vec<row::Row>,
}

impl super::Widget<state::State> for Widget {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;
        self.splits = splits(ctx);
    }

    fn children(&self) -> Vec<&dyn super::Widget<state::State>> {
        self.splits
            .iter()
            .map(|x| x as &dyn super::Widget<state::State>)
            .collect()
    }
}

/// Constructs a vector of split drawing widgets according to `ctx`.
fn splits(ctx: LayoutContext) -> Vec<row::Row> {
    // TODO(@MattWindsor91): padding
    let n_splits = usize::try_from(ctx.bounds.size.h / ctx.wmetrics.split_h).unwrap_or_default();
    (0..n_splits).map(|n| row(ctx, n)).collect()
}

fn row(ctx: LayoutContext, index: usize) -> row::Row {
    let mut r = row::Row::new(index);
    let bounds = Rect {
        top_left: ctx.bounds.point(
            0,
            sat_i32(index) * sat_i32(ctx.wmetrics.split_h),
            Anchor::TOP_LEFT,
        ),
        size: Size {
            w: ctx.bounds.size.w,
            h: ctx.wmetrics.split_h,
        },
    };
    // TODO(@MattWindsor91): maybe make this not depend on <State>?
    widget::Widget::<state::State>::layout(&mut r, ctx.with_bounds(bounds));
    r
}
