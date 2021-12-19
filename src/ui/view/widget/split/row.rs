//! Sub-widget for rendering a split row.

use crate::model::aggregate::{Scope, Source};
use std::fmt::Write;

use super::super::{
    super::{
        presenter::state,
        widget::time::{Colour, FieldColour},
    },
    gfx::{
        self, colour, font,
        metrics::{Anchor, Point, Rect},
        Renderer, Writer,
    },
    layout, time, Widget,
};

/// Contains all state useful to draw one split.
#[derive(Default)]
pub struct Row {
    /// Bounding box used for the widget.
    rect: Rect,
    /// Top-left coordinate of the name.
    name_top_left: Point,
    /// Top-left coordinate of the attempt count.
    attempt_count_top_left: Point,
    /// Layout information for the timer.
    time: time::Layout,
}

impl layout::Layoutable for Row {
    fn layout(&mut self, ctx: layout::Context) {
        self.rect = ctx.bounds;
        self.name_top_left = self.rect.top_left;

        let time_rect = self.time_display_rect(ctx);
        self.time.layout(ctx.with_bounds(time_rect));

        // TODO(@MattWindsor91): de-hardcode the 3 character offset here
        let attempt_offset = ctx.font_metrics[font::Id::Small].span_w(-3);
        self.attempt_count_top_left = time_rect.top_left.offset(attempt_offset, 0);
    }
}

// Row widgets display split state (with the particular allocated split being worked out upstream).
impl Widget for Row {
    type State = state::Split;

    fn render(&self, r: &mut dyn Renderer, s: &Self::State) -> gfx::Result<()> {
        self.draw_name(r, s)?;
        self.draw_num_times(r, s)?;
        self.draw_time_display(r, s)?;
        Ok(())
    }
}

impl Row {
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
        self.time.render(r, Some(e), &col)
    }

    fn draw_time(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        self.time.render(
            r,
            Some(time_to_display(state)),
            &colour::fg::Id::SplitInRunPace(state.pace_in_run).into(),
        )
    }

    fn time_display_rect(&self, ctx: layout::Context) -> Rect {
        self.rect
            .point(0, 0, Anchor::TOP_RIGHT)
            .to_rect(self.time.minimal_size(ctx), Anchor::TOP_RIGHT)
    }

    fn draw_num_times(&self, r: &mut dyn Renderer, state: &state::Split) -> gfx::Result<()> {
        let mut w = Writer::new(r)
            .with_pos(self.attempt_count_top_left)
            .with_font(font::Id::Small.coloured(colour::fg::Id::NoTime)); // for now
        write!(w, "{}x", state.num_times)?;
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
    // TODO(@MattWindsor91): don't hardcode split here
    &state.aggregates[aggregate_source(state)][Scope::Split]
}
