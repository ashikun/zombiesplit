//! The split total widget.

use super::{
    super::{super::presenter::Presenter, error::Result},
    colour, font, metrics,
    position::{Position, X},
    render::{Region, Renderer},
    split::time_str,
    widget,
};
use crate::{model::comparison::pace, ui::presenter};

/// Views the total time for a run.
pub struct Widget {
    /// The bounding box for the header widget.
    pub rect: metrics::Rect,
}

impl widget::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
        let mut r = Region::new(r, self.rect);

        // TODO(@MattWindsor91): clean this up into a struct etc.
        r.set_pos(Position::top_left(0, 0));
        render_total(&mut r, &p.state.footer)?;

        // TODO(@MattWindsor91): don't jiggle the position like this.
        r.set_pos(Position::x(X::Left(0)));
        r.set_pos(Position::rel_chars(&r, 0, 1));
        render_at_cursor(&mut r, &p.state.footer)?;

        Ok(())
    }
}

fn render_total(r: &mut dyn Renderer, s: &presenter::state::Footer) -> Result<()> {
    render_label(r, "Total")?;
    r.set_pos(Position::x(X::Right(0)));
    r.set_font(font::Id::Large);
    render_paced_time(r, s.total)
}

fn render_at_cursor(r: &mut dyn Renderer, s: &presenter::state::Footer) -> Result<()> {
    render_label(r, "Up to cursor")?;
    r.set_font(font::Id::Normal);
    render_paced_time(r, s.at_cursor)
}

/// Logic common to rendering a label.
fn render_label(r: &mut dyn Renderer, label: &str) -> Result<()> {
    r.set_font(font::Id::Normal);
    r.set_fg_colour(colour::fg::Id::Header);
    r.put_str(label)
}

/// Logic common to rendering a paced time.
fn render_paced_time(
    r: &mut dyn Renderer,
    pace::PacedTime { pace, time }: pace::PacedTime,
) -> Result<()> {
    // We don't set a font, because different times have different fonts.
    r.set_fg_colour(colour::fg::Id::Pace(pace));
    r.set_pos(Position::x(X::Right(0)));
    r.put_str_r(&time_str(time))
}
