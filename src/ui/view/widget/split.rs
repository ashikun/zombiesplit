//! Logic for drawing splits.

mod editor;
mod row;

use crate::model::{self, aggregate};
use crate::ui::view::widget;

use super::{
    super::{
        super::presenter::state,
        gfx::{
            self, colour, font,
            metrics::{conv::sat_i32, Anchor, Rect, Size},
            Renderer,
        },
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

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    // TODO(@MattWindsor91): hours?
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}

fn draw_name(r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
    r.set_font(font::Id::Medium);
    r.set_fg_colour(colour::fg::Id::Name(state.position));
    r.put_str(&state.name)?;
    Ok(())
}

fn state_time_str(state: &state::Split) -> String {
    time_str(state.aggregates[aggregate_source(state)][aggregate::Scope::Split])
}

fn time_colour(state: &state::Split) -> colour::fg::Id {
    colour::fg::Id::SplitInRunPace(state.pace_in_run)
}

fn draw_time(r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
    r.set_font(font::Id::Medium);
    r.set_fg_colour(time_colour(state));
    r.put_str(&state_time_str(state))
}

/// Draws a representation of the number of times this split has logged.
fn draw_num_times(r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
    r.set_font(font::Id::Small);
    // TODO(@MattWindsor91): better key?
    r.set_fg_colour(colour::fg::Id::Header);
    r.put_str_r(&format!("{}x", state.num_times))
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
