//! Logic for drawing splits.

use std::convert::TryFrom;

use super::{
    super::{
        super::presenter::{cursor, Presenter},
        error::Result,
    },
    colour,
    font::{self, metrics::TextSizer},
    metrics,
    position::{Position, X},
    render::{Region, Renderer},
};
use crate::{
    model::{self, attempt::split::Set},
    ui::presenter,
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

        for (index, state) in p.state.splits.iter().enumerate() {
            r.set_pos(self.split_pos(index));
            SplitDrawer {
                index,
                state,
                r: &mut r,
                p,
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
struct SplitDrawer<'r, 'g, 'p> {
    index: usize,
    state: &'p presenter::state::Split,
    r: &'r mut Region<'g>,
    p: &'p Presenter<'p>,
}

impl<'r, 'g, 'p> SplitDrawer<'r, 'g, 'p> {
    fn draw(&mut self) -> Result<()> {
        self.draw_name()?;
        self.draw_time()?;
        self.draw_num_times()?;
        Ok(())
    }

    fn draw_name(&mut self) -> Result<()> {
        self.r.set_font(font::Id::Normal);
        self.r.set_fg_colour(colour::fg::Id::Name(self.position()));
        self.r.put_str(self.p.session.name_at(self.index))?;
        Ok(())
    }

    fn draw_time(&mut self) -> Result<()> {
        self.r.set_pos(Position::x(X::Right(0)));
        if 0 < self.num_times() {
            self.draw_summed_time()
        } else {
            self.draw_time_placeholder()
        }
    }

    fn draw_summed_time(&mut self) -> Result<()> {
        // TODO(@MattWindsor91): hours?
        self.r.set_font(font::Id::Normal);
        // TODO(@MattWindsor91): use both dimensions of pace.
        let pair = self.paced_time();
        self.r
            .set_fg_colour(colour::fg::Id::SplitInRunPace(self.state.pace_in_run));
        self.r.put_str_r(&time_str(pair.split.time))
    }

    fn draw_time_placeholder(&mut self) -> Result<()> {
        self.r.set_font(font::Id::Normal);
        self.r.set_fg_colour(colour::fg::Id::NoTime);
        // TODO(@MattWindsor91): tidy this up.
        self.r.put_str_r(&self.time_placeholder())
    }

    fn time_placeholder(&self) -> String {
        self.p
            .session
            .comparison_time_at(self.index)
            .map_or_else(|| "--'--\"---".to_owned(), time_str)
    }

    /// Draws a representation of the number of times this split has logged.
    fn draw_num_times(&mut self) -> Result<()> {
        // TODO(@MattWindsor91): jog to position more accurately.
        self.r.set_pos(Position::x(X::Rel(self.r.span_w(-10))));
        self.r.set_font(font::Id::Small);
        // TODO(@MattWindsor91): better key?
        self.r.set_fg_colour(colour::fg::Id::Header);
        self.r.put_str_r(&format!("{}x", self.num_times()))
    }

    fn position(&self) -> cursor::SplitPosition {
        self.p.split_position(self.index)
    }

    fn paced_time(&self) -> model::comparison::pace::Pair {
        self.p.session.paced_time_at(self.index)
    }

    fn num_times(&self) -> usize {
        self.p.session.num_times_at(self.index)
    }
}

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}
