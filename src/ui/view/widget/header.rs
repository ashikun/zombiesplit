//! Header display.

use super::super::{
    super::{presenter::State, Result},
    gfx::{
        colour, font, metrics::{self, Anchor},
        Renderer,
    },
};
use crate::{model::game::category::{AttemptInfo, Info}};

/// Views information about the run in the form of a header.
pub struct Widget {
    /// The bounding box for the header widget.
    rect: metrics::Rect,
}

impl Widget {
    /// Constructs a new [Widget] using the given layout context.
    pub fn new(ctx: super::LayoutContext) -> Self {
        Self { rect: ctx.bounds }
    }
}

impl super::Widget<State> for Widget {
    fn layout(&mut self, ctx: super::LayoutContext) {
        // TODO(@MattWindsor91): this is the parent bounds set.
        self.rect = ctx.bounds;
    }

    fn render(&self, r: &mut dyn Renderer, s: &State) -> Result<()> {
        r.set_fg_colour(colour::fg::Id::Header);

        self.render_meta(r, &s.game_category)?;
        self.render_attempt(r, &s.attempt)?;
        Ok(())
    }
}

impl Widget {
    fn render_meta(&self, r: &mut dyn Renderer, meta: &Info) -> Result<()> {
        r.set_pos(self.rect.point(0, 0, Anchor::TOP_LEFT));
        r.set_font(font::Id::Large);
        r.put_str(&meta.game)?;

        r.set_pos(self.rect.point(0, r.span_h(1), Anchor::TOP_LEFT));
        r.set_font(font::Id::Normal);
        r.put_str(&meta.category)?;
        Ok(())
    }

    fn render_attempt(&self, r: &mut dyn Renderer, attempt: &AttemptInfo) -> Result<()> {
        r.set_pos(self.rect.point(0, 0, Anchor::TOP_RIGHT));
        r.put_str_r(&format!("#{} ({})", attempt.total, attempt.completed))
    }
}
