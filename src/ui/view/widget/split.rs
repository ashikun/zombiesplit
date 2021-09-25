//! Logic for drawing splits.

use std::convert::TryFrom;

use super::{
    super::{
        super::presenter::{cursor, state},
        error::Result,
        gfx::{
            colour,
            font::{self, metrics::TextSizer},
            metrics,
            position::{Position, X},
            render::{Region, Renderer},
        },
    },
    editor,
};
use crate::model::{self, aggregate};

/// The split viewer widget.
pub struct Widget {
    /// The bounding box used for the widget.
    rect: metrics::Rect,
    /// The height of one split.
    split_h: i32,
}

impl super::Widget<state::State> for Widget {
    fn render(&mut self, r: &mut dyn Renderer, s: &state::State) -> Result<()> {
        let mut r = Region::new(r, self.rect);

        for (index, state) in s.splits.iter().enumerate() {
            SplitDrawer {
                index,
                state,
                r: &mut r,
                split_pos: self.split_pos(index),
                s,
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
    state: &'p state::Split,
    split_pos: Position,
    r: &'r mut Region<'g>,
    s: &'p state::State,
}

impl<'r, 'g, 'p> SplitDrawer<'r, 'g, 'p> {
    fn draw(&mut self) -> Result<()> {
        self.r.set_pos(self.split_pos);
        self.draw_name()?;
        self.draw_time_display()?;
        self.draw_num_times()
    }

    #[allow(clippy::option_if_let_else)]
    fn draw_time_display(&mut self) -> Result<()> {
        let mut r = make_time_display_region(self.r, self.split_pos);
        if let Some(ref state) = self.state.editor {
            editor::Editor { r: &mut r, state }.render()
        } else {
            draw_time(&mut r, self.state)
        }
    }

    fn draw_name(&mut self) -> Result<()> {
        self.r.set_font(font::Id::Normal);
        self.r.set_fg_colour(colour::fg::Id::Name(self.position()));
        self.r.put_str(&self.state.name)?;
        Ok(())
    }

    /// Draws a representation of the number of times this split has logged.
    fn draw_num_times(&mut self) -> Result<()> {
        // TODO(@MattWindsor91): jog to position more accurately.
        self.r.set_pos(Position::x(X::Right(self.r.span_w(10))));
        self.r.set_font(font::Id::Small);
        // TODO(@MattWindsor91): better key?
        self.r.set_fg_colour(colour::fg::Id::Header);
        self.r.put_str_r(&format!("{}x", self.state.num_times))
    }

    fn position(&self) -> cursor::SplitPosition {
        self.s.split_position(self.index)
    }
}

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    // TODO(@MattWindsor91): hours?
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}

fn state_time_str(state: &state::Split) -> String {
    time_str(state.aggregates[aggregate_source(state)][aggregate::Scope::Split])
}

fn time_colour(state: &state::Split) -> colour::fg::Id {
    colour::fg::Id::SplitInRunPace(state.pace_in_run)
}

fn make_time_display_region<'a>(parent: &'a mut Region, split_pos: Position) -> Region<'a> {
    // work out where the x position of the time display is
    let parent_size = parent.size();
    let size = editor::size(parent);
    let x = metrics::sat_i32(parent_size.w - size.w);

    let rect = metrics::Rect {
        x,
        y: split_pos.y.to_top(0, 0),
        size,
    };

    Region::new(parent, rect)
}

fn draw_time(r: &mut Region, state: &state::Split) -> Result<()> {
    r.set_pos(Position::top_left(0, 0));
    r.set_font(font::Id::Normal);
    r.set_fg_colour(time_colour(state));
    r.put_str(&state_time_str(state))
}

/// Decide which source to use for the aggregate displayed on this split.
///
/// Currently, this is always the comparison if there are no times logged
/// for the attempt, and the attempt otherwise.
fn aggregate_source(state: &state::Split) -> aggregate::Source {
    if 0 < state.num_times {
        aggregate::Source::Attempt
    } else {
        aggregate::Source::Comparison
    }
}
