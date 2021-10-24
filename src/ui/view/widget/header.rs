//! Header display.

use std::io::Write;

use super::super::{
    super::presenter::State,
    gfx::{
        self, colour, font,
        metrics::{self, Anchor},
        Renderer, Writer,
    },
};
use crate::model::game::category::{AttemptInfo, Info};

/// Views information about the run in the form of a header.
#[derive(Default)]
pub struct Widget {
    /// The bounding box for the header widget.
    rect: metrics::Rect,

    /// The position of the category name.
    category_pos: metrics::Point,
}

impl super::Widget<State> for Widget {
    fn layout(&mut self, ctx: super::LayoutContext) {
        // TODO(@MattWindsor91): this is the parent bounds set.
        self.rect = ctx.bounds;

        let header_metrics = &ctx.font_metrics[HEADER_FONT_SPEC.id];

        self.category_pos = self.rect.point(
            0,
            header_metrics.span_h(1),
            super::metrics::Anchor::TOP_LEFT,
        );
    }

    fn render(&self, r: &mut dyn Renderer, s: &State) -> gfx::Result<()> {
        r.set_fg_colour(colour::fg::Id::Header);

        self.render_meta(r, &s.game_category)?;
        self.render_attempt(r, &s.attempt)?;
        Ok(())
    }
}

const HEADER_FONT_SPEC: font::Spec = font::Spec {
    id: font::Id::Large,
    colour: colour::fg::Id::Header,
};

const CATEGORY_FONT_SPEC: font::Spec = font::Spec {
    id: font::Id::Medium,
    colour: colour::fg::Id::Header,
};

impl Widget {
    fn render_meta(&self, r: &mut dyn Renderer, meta: &Info) -> gfx::Result<()> {
        self.write_header(r, meta)?;

        r.set_font(font::Id::Medium);
        r.set_pos(self.rect.point(
            0,
            r.font_metrics()[font::Id::Medium].span_h(-1),
            Anchor::BOTTOM_LEFT,
        ));
        r.put_str(&meta.category)?;
        Ok(())
    }

    fn write_header(&self, r: &mut dyn Renderer, meta: &Info) -> gfx::Result<()> {
        let mut w = Writer::new(r)
            .with_font(HEADER_FONT_SPEC)
            .with_pos(self.rect.top_left);
        write!(w, "{}", meta.game)?;
        Ok(())
    }

    fn render_attempt(&self, r: &mut dyn Renderer, attempt: &AttemptInfo) -> gfx::Result<()> {
        r.set_pos(self.rect.point(
            0,
            r.font_metrics()[font::Id::Medium].span_h(-1),
            Anchor::BOTTOM_RIGHT,
        ));
        r.put_str_r(&format!("#{} ({})", attempt.total, attempt.completed))
    }
}
