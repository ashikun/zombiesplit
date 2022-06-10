//! Footer rows.

use std::borrow::Cow;
use ugly::{metrics, resource::Map};

use super::super::{
    super::{
        super::{
            super::model::timing::comparison,
            presenter::state::{self, footer},
        },
        config,
        gfx::{colour, font, Renderer},
        layout::{self, Layoutable},
        update::{self, Updatable},
    },
    time, Label, Widget,
};

/// Sub-widget for a row in the footer.
pub struct Row {
    bounds: metrics::Rect,

    /// The type of row being shown here.
    row_type: footer::RowType,

    /// The label widget.
    label: Label,
    /// The time widget.
    time: time::Layout,
}

impl Row {
    /// Constructs a row with the given initial layout information and no set positioning.
    pub(super) fn new(layout: &config::layout::Row) -> Self {
        let mut time = time::Layout::default();
        time.font_id = layout.font;
        Self {
            bounds: metrics::Rect::default(),
            row_type: layout.contents,
            label: Label::new(font::Spec::default()),
            time,
        }
    }

    fn update_time(&mut self, ctx: &update::Context, time: &Option<Cow<comparison::PacedTime>>) {
        let pace = time
            .as_ref()
            .map_or_else(comparison::Pace::default, |t| t.pace);

        let t = time.as_ref().map(|x| &x.as_ref().time);

        self.time.update(ctx, t, colour::fg::Id::Pace(pace).into());
    }
}

impl Layoutable for Row {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        // TODO(@MattWindsor91): restructure this as a stack?
        metrics::Size::stack_horizontally(label_size(parent_ctx), self.time.min_bounds(parent_ctx))
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        self.label.layout(ctx);

        let time_rect = ctx
            .bounds
            .anchor(metrics::Anchor::TOP_RIGHT)
            .to_rect(self.time.min_bounds(ctx), metrics::Anchor::TOP_RIGHT);
        self.time.layout(ctx.with_bounds(time_rect));
    }
}

fn label_size(ctx: layout::Context) -> metrics::Size {
    ctx.font_metrics.get(font::Id::Medium).text_size(0, 1)
}

impl Updatable for Row {
    type State = state::Footer;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        self.label.update(ctx, self.row_type.label());
        self.update_time(ctx, &s.get(self.row_type));
    }
}

impl<'r, R: Renderer<'r>> Widget<R> for Row {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.label.render(r)?;
        self.time.render(r)?;
        Ok(())
    }
}
