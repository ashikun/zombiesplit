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
    model::{self, aggregate},
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
        self.r.put_str(&self.state.name)?;
        Ok(())
    }

    fn draw_time(&mut self) -> Result<()> {
        self.r.set_pos(Position::x(X::Right(0)));
        self.r.set_fg_colour(self.time_colour());
        self.r.put_str_r(&self.time_str())
    }

    fn time_str(&self) -> String {
        time_str(self.state.aggregates[self.aggregate_source()][aggregate::Scope::Split])
    }

    fn time_colour(&self) -> colour::fg::Id {
        colour::fg::Id::SplitInRunPace(self.state.pace_in_run)
    }

    /// Decide which source to use for the aggregate displayed on this split.
    ///
    /// Currently, this is always the comparison if there are no times logged
    /// for the attempt, and the attempt otherwise.
    fn aggregate_source(&self) -> aggregate::Source {
        if 0 < self.state.num_times {
            aggregate::Source::Attempt
        } else {
            aggregate::Source::Comparison
        }
    }

    /// Draws a representation of the number of times this split has logged.
    fn draw_num_times(&mut self) -> Result<()> {
        // TODO(@MattWindsor91): jog to position more accurately.
        self.r.set_pos(Position::x(X::Rel(self.r.span_w(-10))));
        self.r.set_font(font::Id::Small);
        // TODO(@MattWindsor91): better key?
        self.r.set_fg_colour(colour::fg::Id::Header);
        self.r.put_str_r(&format!("{}x", self.state.num_times))
    }

    fn position(&self) -> cursor::SplitPosition {
        self.p.split_position(self.index)
    }
}

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    // TODO(@MattWindsor91): hours?
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}
