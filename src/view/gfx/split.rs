//! Logic for drawing splits.

use std::convert::TryFrom;

use super::{
    colour, metrics,
    position::{Position, X},
    render::{FontId, Region, Renderer},
};
use crate::{
    model,
    presenter::{split, Presenter},
    view::error::Result,
};

/// The split viewer widget.
pub struct Widget {
    /// The bounding box used for the widget.
    rect: metrics::Rect,
    /// The height of one split.
    split_h: i32,
}

impl super::widget::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
        let mut r = Region::new(r, self.rect);

        for split in p.splits() {
            r.set_pos(self.split_pos(split));
            draw_split(&mut r, split)?;
        }
        Ok(())
    }
}

impl Widget {
    /// Creates a new view using the given bounding box and split height.
    pub fn new(rect: metrics::Rect, split_h: i32) -> Self {
        Self { rect, split_h }
    }

    fn split_pos(&self, split: split::Ref) -> Position {
        Position::top_left(
            0,
            i32::try_from(split.index).unwrap_or_default() * self.split_h,
        )
    }
}

fn draw_split(r: &mut Region, split: split::Ref) -> Result<()> {
    draw_name(r, split)?;
    draw_time(r, split)?;
    Ok(())
}

fn draw_name(r: &mut Region, split: split::Ref) -> Result<()> {
    let colour = colour::Key::Name(split.position());
    r.set_font(FontId::Normal(colour));
    r.put_str(&split.split.name)?;
    Ok(())
}

fn draw_time(r: &mut Region, split: split::Ref) -> Result<()> {
    r.set_pos(Position::x(X::Right(0)));
    if split.split.has_times() {
        draw_summed_time(r, split.split.summed_time())
    } else {
        draw_time_placeholder(r)
    }
}

fn draw_summed_time(r: &mut Region, time: model::time::Time) -> Result<()> {
    let colour = colour::Key::RunAhead; // for now

    // TODO(@MattWindsor91): hours?
    r.set_font(FontId::Normal(colour));
    r.put_str_r(&time_str(time))
}

fn draw_time_placeholder(r: &mut Region) -> Result<()> {
    r.set_font(FontId::Normal(colour::Key::NoTime));
    r.put_str_r("--'--\"---")
}

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}
