//! Sub-widget for rendering a split row.

use ugly::metrics;

use super::super::{
    super::{
        super::super::model::timing::{
            aggregate::{Scope, Source},
            comparison::pace::SplitInRun,
        },
        gfx::{colour, font, Renderer},
        layout::{self, Layoutable},
        presenter::state,
        update::{self, Updatable},
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
    /// The attempt count label.
    attempt_count: Label,
    /// Layout information for the timer.
    time: time::Layout,
}

impl Default for Row {
    fn default() -> Self {
        Self {
            bounds: metrics::Rect::default(),
            rect: metrics::Rect::default(),
            name: Label::new(NAME_FONT_SPEC).min_chars(NAME_MIN_CHARS),
            attempt_count: Label::new(ATTEMPT_COUNT_FONT_SPEC).min_chars(ATTEMPT_COUNT_MIN_CHARS),
            time: time::Layout::default(),
        }
    }
}

const ATTEMPT_COUNT_FONT_SPEC: font::Spec = font::Spec {
    id: font::Id::Small,
    colour: colour::fg::Id::Normal,
};
const ATTEMPT_COUNT_MIN_CHARS: u8 = 3;
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
                self.attempt_count.min_bounds(parent_ctx),
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

        let attempt_size = ctx
            .font_metrics(font::Id::Small)
            .text_size(ATTEMPT_COUNT_LENGTH, 1);
        let attempt_rect = time_rect
            .top_left
            .to_rect(attempt_size, metrics::Anchor::TOP_RIGHT);
        self.attempt_count.layout(ctx.with_bounds(attempt_rect));
    }
}

// Row widgets display split state (with the particular allocated split being worked out upstream).
impl Updatable for Row {
    type State = state::Split;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        self.update_name(ctx, s);
        self.update_num_times(ctx, s);
        self.update_time_display(ctx, s);
    }
}

impl<'r, R: Renderer<'r>> Widget<R> for Row {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.name.render(r)?;
        self.attempt_count.render(r)?;
        self.time.render(r)?;
        Ok(())
    }
}

impl Row {
    fn update_name(&mut self, ctx: &update::Context, state: &state::Split) {
        let colour = colour::fg::Id::Name(state.position);
        self.name.update_extended(ctx, &state.name, colour);
    }

    fn update_time_display(&mut self, ctx: &update::Context, state: &state::Split) {
        if let Some(ref e) = state.editor {
            self.update_editor(ctx, e);
        } else {
            self.update_time(ctx, state);
        }
    }

    fn update_editor(&mut self, ctx: &update::Context, e: &state::editor::Editor) {
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
        self.time.update(ctx, Some(e), col);
    }

    fn update_time(&mut self, ctx: &update::Context, state: &state::Split) {
        self.time.update(
            ctx,
            Some(time_to_display(state)),
            colour::fg::Id::SplitInRunPace(
                state.delta.map_or(SplitInRun::Inconclusive, |d| d.pace),
            )
            .into(),
        );
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

    fn update_num_times(&mut self, ctx: &update::Context, state: &state::Split) {
        self.attempt_count
            .update(ctx, &format!("{}x", state.times.len()));
    }
}

fn name_size(parent_ctx: layout::Context) -> metrics::Size {
    parent_ctx.font_metrics(font::Id::Medium).text_size(0, 1)
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
