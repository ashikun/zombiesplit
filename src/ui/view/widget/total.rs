//! The split total widget.

use super::{
    super::{
        super::{presenter::state, Result},
        gfx::{
            colour, font, metrics::{self, Anchor},
            Renderer,
        },
    },
    split::time_str,
};
use crate::model::comparison::pace;

/// Views the total time for a run.
pub struct Widget {
    /// The bounding box for the footer widget.
    pub rect: metrics::Rect,
}

impl Widget {
    /// Constructs a new [Widget] using the given layout context.
    pub fn new(ctx: super::LayoutContext) -> Self {
        Self { rect: ctx.bounds }
    }
}

impl super::Widget<state::State> for Widget {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::State) -> Result<()> {
        r.set_pos(self.rect.point(0, 0, Anchor::TOP_LEFT));
        self.render_total(r, &s.footer)?;

        r.set_pos(self.rect.point(0, r.span_h(1), Anchor::TOP_LEFT));
        self.render_at_cursor(r, &s.footer)?;

        Ok(())
    }
}

impl Widget {
    fn render_total(&self, r: &mut dyn Renderer, s: &state::Footer) -> Result<()> {
        render_label(r, "Total")?;
        r.set_font(font::Id::Large);
        self.render_paced_time(r, s.total)
    }

    fn render_at_cursor(&self, r: &mut dyn Renderer, s: &state::Footer) -> Result<()> {
        render_label(r, "Up to cursor")?;
        // TODO(@MattWindsor91): positioning hack here.
        r.set_font(font::Id::Large);
        r.set_pos(self.rect.point(0, r.span_h(1), Anchor::TOP_RIGHT));
        r.set_font(font::Id::Normal);
        self.render_paced_time(r, s.at_cursor)
    }

    /// Logic common to rendering a paced time.
    fn render_paced_time(
        &self,
        r: &mut dyn Renderer,
        pace::PacedTime { pace, time }: pace::PacedTime,
    ) -> Result<()> {
        // We don't set a font, because different times have different fonts.
        r.set_fg_colour(colour::fg::Id::Pace(pace));
        r.put_str_r(&time_str(time))
    }
}

/// Logic common to rendering a label.
fn render_label(r: &mut dyn Renderer, label: &str) -> Result<()> {
    r.set_font(font::Id::Normal);
    r.set_fg_colour(colour::fg::Id::Header);
    r.put_str(label)
}

