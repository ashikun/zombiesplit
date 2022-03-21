//! Sub-widget for rendering a split row.

use std::fmt::Write;
use ugly::metrics;

use super::super::{
    super::{
        super::super::model::timing::aggregate::{Scope, Source},
        gfx::{colour, font, Renderer},
        layout::{self, Layoutable},
        presenter::state,
    },
    label::Label,
    time, Widget,
};

/// Contains all state useful to draw one split.
#[derive(Clone)]
pub struct Row {
    /// Outer bounding box used for the widget.
    bounds: metrics::Rect,
    /// Inner, padded, bounding box used for the widget.
    rect: metrics::Rect,
    /// The name label.
    name: Label,
    /// Top-left coordinate of the attempt count.
    attempt_count_top_left: metrics::Point,
    /// Layout information for the timer.
    time: time::Layout,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            bounds: metrics::Rect::default(),
            rect: metrics::Rect::default(),
            name: Label::new(NAME_FONT_SPEC).min_chars(NAME_MIN_CHARS),
            attempt_count_top_left: metrics::Point::default(),
            time: time::Layout::default(),
        }
    }
}

const NAME_FONT_SPEC: font::Spec = font::Spec {
    id: font::Id::Medium,
    colour: colour::fg::Id::Name(state::cursor::SplitPosition::Coming),
};
const NAME_MIN_CHARS: u8 = 10;

impl Layoutable for Row {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        metrics::Size::stack_horizontally(
            name_size(parent_ctx),
            metrics::Size::stack_horizontally(
                attempt_count_size(parent_ctx),
                self.time.min_bounds(parent_ctx),
            ),
        )
        .grow(2 * parent_ctx.config.window.padding)
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;
        self.rect = ctx.padded().bounds;

        let name_rect = self.name_rect(ctx);
        self.name.layout(ctx.with_bounds(name_rect));

        let time_rect = self.time_display_rect(ctx);
        self.time.layout(ctx.with_bounds(time_rect));

        let attempt_offset = ctx
            .font_metrics(font::Id::Small)
            .span_w(-ATTEMPT_COUNT_LENGTH);
        self.attempt_count_top_left = time_rect.top_left.offset(attempt_offset, 0);
    }
}

// Row widgets display split state (with the particular allocated split being worked out upstream).
impl<R: Renderer> Widget<R> for Row {
    type State = state::Split;

    fn render(&self, r: &mut R, s: &Self::State) -> ugly::Result<()> {
        self.draw_name(r, s)?;
        self.draw_num_times(r, s)?;
        self.draw_time_display(r, s)?;
        Ok(())
    }
}

impl Row {
    fn draw_name(&self, r: &mut impl Renderer, state: &state::Split) -> ugly::Result<()> {
        let colour = colour::fg::Id::Name(state.position);
        self.name.render_extended(r, &state.name, colour)
    }

    fn draw_time_display(&self, r: &mut impl Renderer, state: &state::Split) -> ugly::Result<()> {
        if let Some(ref e) = state.editor {
            self.draw_editor(r, e)
        } else {
            self.draw_time(r, state)
        }
    }

    fn draw_editor(&self, r: &mut impl Renderer, e: &state::editor::Editor) -> ugly::Result<()> {
        let field = e.field.map(|field| time::FieldColour {
            field,
            colour: colour::Pair {
                fg: colour::fg::Id::FieldEditor,
                bg: Some(colour::bg::Id::FieldEditor),
            },
        });
        let col = time::Colour {
            base: colour::Pair {
                fg: colour::fg::Id::Editor,
                bg: Some(colour::bg::Id::Editor),
            },
            field,
        };
        self.time.render(r, Some(e), &col)
    }

    fn draw_time(&self, r: &mut impl Renderer, state: &state::Split) -> ugly::Result<()> {
        self.time.render(
            r,
            Some(time_to_display(state)),
            &colour::fg::Id::SplitInRunPace(state.pace_in_run).into(),
        )
    }

    fn time_display_rect(&self, ctx: layout::Context) -> metrics::Rect {
        self.rect
            .anchor(metrics::Anchor::TOP_RIGHT)
            .to_rect(self.time.min_bounds(ctx), metrics::Anchor::TOP_RIGHT)
    }

    fn name_rect(&self, ctx: layout::Context) -> metrics::Rect {
        let mut r = self.rect;
        r.size.h = self.name.min_bounds(ctx).h;
        r
    }

    fn draw_num_times(&self, r: &mut impl Renderer, state: &state::Split) -> ugly::Result<()> {
        let mut w = ugly::text::Writer::new(r)
            .with_pos(self.attempt_count_top_left)
            .with_font(font::Id::Small.coloured(colour::fg::Id::Normal)); // for now
        write!(w, "{}x", state.times.len())?;
        Ok(())
    }
}

fn name_size(parent_ctx: layout::Context) -> metrics::Size {
    parent_ctx.font_metrics(font::Id::Medium).text_size(0, 1)
}

fn attempt_count_size(parent_ctx: layout::Context) -> metrics::Size {
    parent_ctx
        .font_metrics(font::Id::Small)
        .text_size(ATTEMPT_COUNT_LENGTH, 1)
}

// TODO(@MattWindsor91): de-hardcode this
const ATTEMPT_COUNT_LENGTH: i32 = 3; // #xx

/// Decide which source to use for the aggregate displayed on this split.
///
/// Currently, this is always the comparison if there are no times logged
/// for the attempt, and the attempt otherwise.
fn aggregate_source(state: &state::Split) -> Source {
    if state.times.is_empty() {
        Source::Comparison
    } else {
        Source::Attempt
    }
}

fn time_to_display(state: &state::Split) -> &crate::model::Time {
    // TODO(@MattWindsor91): don't hardcode split here
    &state.aggregates[aggregate_source(state)][Scope::Split]
}
