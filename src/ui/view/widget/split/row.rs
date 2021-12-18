//! Sub-widget for rendering a split row.

use crate::model::aggregate::{Scope, Source};
use std::fmt::Write;

use super::{
    super::{
        super::{
            presenter::state,
            widget::time::{Colour, FieldColour},
        },
        gfx::{
            self, colour, font,
            metrics::{conv::sat_i32, Anchor, Point, Rect},
            Renderer, Writer,
        },
        time, Widget,
    },
    LayoutContext,
};

/// Contains all state useful to draw one split.
#[derive(Default)]
pub struct Row {
    /// The position of the drawer in the split view.
    /// This is not necessarily the index of the split displayed.
    index: usize,
    /// Bounding box used for the widget.
    rect: Rect,
    /// Top-left coordinate of the name.
    name_top_left: Point,
    /// Top-left coordinate of the attempt count.
    attempt_count_top_left: Point,
    /// Layout information for the timer.
    time: time::Layout,
}

/// Use of a row when we have not resolved which split is inside the drawer.
impl Widget<state::State> for Row {
    fn layout(&mut self, ctx: super::LayoutContext) {
        Widget::<state::Split>::layout(self, ctx);
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::State) -> gfx::Result<()> {
        // TODO(@MattWindsor91): calculate which split should be here based on
        // cursor position.
        if let Some(split) = s.splits.get(self.index) {
            Widget::<state::Split>::render(self, r, split)?;
        }

        Ok(())
    }
}

/// Use of a split widget when we have resolved which split is inside the drawer.
impl Widget<state::Split> for Row {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;
        self.name_top_left = self.rect.top_left;

        let time_rect = self.time_display_rect(ctx);
        self.time.update(ctx.with_bounds(time_rect));

        // TODO(@MattWindsor91): de-hardcode the 2 character offset here
        let attempt_offset = ctx.font_metrics[font::Id::Small].span_w(-2);
        self.attempt_count_top_left = time_rect.top_left.offset(attempt_offset, 0);
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::Split) -> gfx::Result<()> {
        self.draw_name(r, s)?;
        self.draw_num_times(r, s)?;
        self.draw_time_display(r, s)?;
        Ok(())
    }
}

impl Row {
    /// Constructs a new split row for displaying split position `index`.
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Self::default()
        }
    }

    fn draw_name(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        let colour = colour::fg::Id::Name(state.position);

        Writer::new(r)
            .with_pos(self.name_top_left)
            .with_font(font::Id::Medium.coloured(colour))
            .write_str(&state.name)?;
        Ok(())
    }

    fn draw_time_display(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        if let Some(ref e) = state.editor {
            self.draw_editor(r, e)
        } else {
            self.draw_time(r, state)
        }
    }

    fn draw_editor(&self, r: &mut dyn Renderer, e: &state::Editor) -> gfx::Result<()> {
        let field = e.field.map(|field| FieldColour {
            field,
            colour: colour::Pair {
                fg: colour::fg::Id::Editor,
                bg: Some(colour::bg::Id::Editor),
            },
        });
        let col = Colour {
            base: colour::Pair {
                fg: colour::fg::Id::FieldEditor,
                bg: Some(colour::bg::Id::FieldEditor),
            },
            field,
        };
        self.time.render(r, e, &col)
    }

    fn draw_time(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        self.time.render(
            r,
            time_to_display(state),
            &colour::fg::Id::SplitInRunPace(state.pace_in_run).into(),
        )
    }

    fn time_display_rect(&self, ctx: LayoutContext) -> Rect {
        // Need to precalculate the size so we can jog the layout backwards by that amount.
        let size = time::size(ctx, font::Id::Medium);
        Rect {
            top_left: self.rect.point(-sat_i32(size.w), 0, Anchor::TOP_RIGHT),
            size,
        }
    }

    fn draw_num_times(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        let mut w = Writer::new(r)
            .with_pos(self.attempt_count_top_left)
            .with_font(font::Id::Small.coloured(colour::fg::Id::NoTime)); // for now
        write!(w, "{}", state.num_times)?;
        Ok(())
    }
}

/// Decide which source to use for the aggregate displayed on this split.
///
/// Currently, this is always the comparison if there are no times logged
/// for the attempt, and the attempt otherwise.
fn aggregate_source(state: &state::Split) -> Source {
    if 0 < state.num_times {
        Source::Attempt
    } else {
        Source::Comparison
    }
}

fn time_to_display(state: &state::Split) -> &crate::model::Time {
    // TODO(@MattWindsor91): don't hardcode cumulative here
    &state.aggregates[aggregate_source(state)][Scope::Cumulative]
}
