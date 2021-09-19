//! Header display.

use super::super::{
    super::presenter,
    error::Result,
    gfx::{
        colour, font, metrics,
        position::{Position, X},
        render::{Region, Renderer},
    },
};
use crate::model::game::category::{AttemptInfo, Info};

/// Views information about the run in the form of a header.
pub struct Widget {
    /// The bounding box for the header widget.
    pub rect: metrics::Rect,
}

impl super::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &presenter::Core) -> Result<()> {
        let mut r = Region::new(r, self.rect);

        r.set_fg_colour(colour::fg::Id::Header);

        render_meta(&mut r, &p.state.game_category)?;
        render_attempt(&mut r, &p.state.attempt)?;
        Ok(())
    }
}

fn render_meta(r: &mut dyn Renderer, meta: &Info) -> Result<()> {
    r.set_pos(Position::top_left(0, 0));
    r.set_font(font::Id::Large);
    r.put_str(&meta.game)?;
    r.set_pos(Position::rel_chars(r, 0, 1));
    r.set_font(font::Id::Normal);
    r.put_str(&meta.category)?;
    Ok(())
}

fn render_attempt(r: &mut dyn Renderer, attempt: &AttemptInfo) -> Result<()> {
    r.set_pos(Position::x(X::Right(0)));
    r.put_str_r(&format!("#{} ({})", attempt.total, attempt.completed))
}
