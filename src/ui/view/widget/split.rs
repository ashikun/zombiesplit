//! Logic for drawing splits.

use super::{
    super::{
        super::presenter::state,
        gfx::{
            self, colour, font,
            metrics::{conv::sat_i32, Anchor, Rect, Size},
            Renderer,
        },
    },
    editor, LayoutContext,
};
use crate::model::{self, aggregate};

/// The split viewer widget.
#[derive(Default)]
pub struct Widget {
    /// The bounding box used for the widget.
    rect: Rect,
    /// The split drawer set, containing enough drawers for one layout.
    splits: Vec<SplitDrawer>,
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

/// Contains all state useful to draw one split.
struct SplitDrawer {
    /// The position of the drawer in the split view.
    /// This is not necessarily the index of the split displayed.
    index: usize,
    /// The bounding box used for the widget.
    rect: Rect,
}

impl super::Widget<state::State> for SplitDrawer {
    fn layout(&mut self, ctx: super::LayoutContext) {
        super::Widget::<state::Split>::layout(self, ctx);
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::State) -> gfx::Result<()> {
        // TODO(@MattWindsor91): calculate which split should be here based on
        // cursor position.
        if let Some(split) = s.splits.get(self.index) {
            super::Widget::<state::Split>::render(self, r, split)?;
        }

        Ok(())
    }
}

/// Use of a split widget when we have resolved which split is inside the drawer.
impl super::Widget<state::Split> for SplitDrawer {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::Split) -> gfx::Result<()> {
        r.set_pos(self.rect.top_left);
        draw_name(r, s)?;
        self.draw_time_display(r, s)?;

        // TODO(@MattWindsor91): jog to position more accurately.
        r.set_pos(self.rect.point(
            r.font_metrics()[font::Id::Medium].span_w(-10),
            0,
            Anchor::TOP_RIGHT,
        ));
        draw_num_times(r, s)?;
        Ok(())
    }
}

impl SplitDrawer {
    #[allow(clippy::option_if_let_else)]
    fn draw_time_display(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        let rect = self.time_display_rect(r);
        r.set_pos(rect.top_left);
        if let Some(ref state) = state.editor {
            use super::Widget;
            editor::Editor { rect }.render(r, state)
        } else {
            Ok(draw_time(r, state)?)
        }
    }

    fn time_display_rect(&self, r: &mut dyn Renderer) -> Rect {
        let size = editor::size(&r.font_metrics()[font::Id::Medium]);
        Rect {
            top_left: self.rect.point(-sat_i32(size.w), 0, Anchor::TOP_RIGHT),
            size,
        }
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

/// Decide which source to use for the aggregate displayed on this split.
///
/// Currently, this is always the comparison if there are no times logged
/// for the attempt, and the attempt otherwise.
fn aggregate_source(state: &state::Split) -> aggregate::Source {
    if 0 < state.num_times {
        aggregate::Source::Attempt
    } else {
        aggregate::Source::Comparison
    }
}

/// Constructs a vector of split drawing widgets according to `ctx`.
fn splits(ctx: LayoutContext) -> Vec<SplitDrawer> {
    // TODO(@MattWindsor91): padding
    let n_splits = usize::try_from(ctx.bounds.size.h / ctx.wmetrics.split_h).unwrap_or_default();
    (0..n_splits)
        .map(|n| SplitDrawer {
            index: n,
            rect: Rect {
                top_left: ctx.bounds.point(
                    0,
                    sat_i32(n) * sat_i32(ctx.wmetrics.split_h),
                    Anchor::TOP_LEFT,
                ),
                size: Size {
                    w: ctx.bounds.size.w,
                    h: ctx.wmetrics.split_h,
                },
            },
        })
        .collect()
}
