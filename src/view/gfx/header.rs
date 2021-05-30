//! Header display.

use super::{
    colour, metrics,
    position::{Position, X},
    render::{FontId, Region, Renderer},
    widget,
};
use crate::{presenter::Presenter, view::error::Result};

/// Views information about the run in the form of a header.
pub struct Widget {
    /// The bounding box for the header widget.
    pub rect: metrics::Rect,
}

impl widget::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
        let mut r = Region::new(r, self.rect);

        r.set_pos(Position::top_left(0, 0));
        r.set_font(FontId::Normal(colour::Key::Header));
        r.put_str(&p.run.metadata.game)?;
        r.move_chars(0, 1);
        r.put_str(&p.run.metadata.category)?;

        render_attempt(&mut r, p.run.attempt)?;
        Ok(())
    }
}

fn render_attempt(r: &mut dyn Renderer, attempt: usize) -> Result<()> {
    r.set_pos(Position::x(X::Right(0)));
    r.put_str_r(&format!("#{}", attempt))
}
