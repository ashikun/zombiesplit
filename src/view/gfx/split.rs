//! Logic for drawing splits.

use std::convert::TryFrom;

use super::{
    colour, font, metrics,
    position::{Position, X},
    render::{Region, Renderer},
};
use crate::{
    model,
    presenter::{cursor, Presenter},
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

        for (index, split) in p.run.splits.iter().enumerate() {
            r.set_pos(self.split_pos(index));
            SplitDrawer {
                index,
                r: &mut r,
                p,
                split,
            }
            .draw()?;
        }
        Ok(())
    }
}

impl Widget {
    /// Creates a new view using the given bounding box and split height.
    pub fn new(rect: metrics::Rect, split_h: i32) -> Self {
        Self { rect, split_h }
    }

    fn split_pos(&self, index: usize) -> Position {
        Position::top_left(0, i32::try_from(index).unwrap_or_default() * self.split_h)
    }
}

/// Contains all state useful to draw one split.
struct SplitDrawer<'r, 'g, 'p, 's> {
    index: usize,
    r: &'r mut Region<'g>,
    p: &'p Presenter,
    split: &'s model::split::Split,
}

impl<'r, 'g, 'p, 's> SplitDrawer<'r, 'g, 'p, 's> {
    fn draw(&mut self) -> Result<()> {
        self.draw_name()?;
        self.draw_time()?;
        Ok(())
    }

    fn draw_name(&mut self) -> Result<()> {
        self.r.set_font(font::Id::Normal);
        self.r.set_fg_colour(colour::Key::Name(self.position()));
        self.r.put_str(&self.split.name)?;
        Ok(())
    }

    fn draw_time(&mut self) -> Result<()> {
        self.r.set_pos(Position::x(X::Right(0)));
        if self.split.has_times() {
            self.draw_summed_time()
        } else {
            self.draw_time_placeholder()
        }
    }

    fn draw_summed_time(&mut self) -> Result<()> {
        // TODO(@MattWindsor91): hours?
        self.r.set_font(font::Id::Normal);
        // TODO(@MattWindsor91): use both dimensions of pace.
        let model::pace::Pair { split, .. } = self.paced_time();
        self.r.set_fg_colour(colour::Key::Pace(split.pace));
        self.r.put_str_r(&time_str(split.time))
    }

    fn draw_time_placeholder(&mut self) -> Result<()> {
        self.r.set_font(font::Id::Normal);
        self.r.set_fg_colour(colour::Key::NoTime);
        self.r.put_str_r("--'--\"---")
    }

    fn position(&self) -> cursor::SplitPosition {
        self.p.split_position(self.index)
    }

    fn paced_time(&self) -> model::pace::Pair {
        self.p.run.paced_time_at(self.index)
    }
}

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}
