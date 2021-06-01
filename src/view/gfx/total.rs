//! The split total widget.

use super::{
    colour, font, metrics,
    position::{Position, X},
    render::{Region, Renderer},
    split::time_str,
    widget,
};
use crate::{model::pace, presenter::Presenter, view::error::Result};

/// Views the total time for a run.
pub struct Widget {
    /// The bounding box for the header widget.
    pub rect: metrics::Rect,
}

impl widget::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
        let mut r = Region::new(r, self.rect);

        render_label(&mut r)?;
        render_time(&mut r, p)
    }
}

fn render_label(r: &mut dyn Renderer) -> Result<()> {
    r.set_pos(Position::top_left(0, 0));
    r.set_font(font::Id::Normal)?;
    r.set_fg_colour(colour::Key::Header);
    r.put_str("Total after cursor")
}

fn render_time(r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
    r.set_pos(Position::x(X::Right(0)));
    // TODO(@MattWindsor91): large font?
    r.set_font(font::Id::Normal)?;
    let pace::Pair {
        run_so_far: pace::PacedTime { pace, time },
        ..
    } = p.run_pace();
    r.set_fg_colour(colour::Key::Pace(pace));
    r.put_str_r(&time_str(time))
}
