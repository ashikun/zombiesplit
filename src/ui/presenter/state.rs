/*! The presenter's internal state.

This is populated from the model every time the presenter observes some kind
of change on the model.
*/

use std::ops::{Index, IndexMut};

pub use footer::Footer;
pub use split::Split;

use crate::model::{
    attempt,
    game::category,
    short,
    timing::{aggregate, comparison::pace::PacedTime, time},
};

pub mod cursor;
pub mod footer;
pub mod split;

/// The presenter's representation of the model.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct State {
    /// The cursor (not made public so as to ensure that we update the splits and footer).
    cursor: cursor::Cursor,

    /// The current attempt information.
    pub attempt: category::AttemptInfo,
    /// Information about the game and category being played.
    pub game_category: category::Info,

    /// State for the splits being displayed in the UI.
    splits: split::Set,

    /// State for the footer widget.
    pub footer: Footer,
}

impl State {
    /// Makes the presenter state reflect a reset in the run.
    ///
    /// This clears the time count aggregate data for all splits.  It doesn't
    /// change the attempt information, as we expect there will be a separate
    /// observation for that.
    pub fn reset(&mut self) {
        self.cursor.reset();
        self.splits.refresh_cursors(&self.cursor);
        self.splits.reset();
        self.footer.total = PacedTime::default();
    }

    /// Gets the current position of the cursor.
    #[must_use]
    pub fn cursor_position(&self) -> usize {
        self.cursor.position()
    }

    /// Moves the cursor in the direction of `m` `multiplier` times.
    /// Returns the absolute amount by which the cursor moved.
    pub fn move_cursor_by(&mut self, m: cursor::Motion, multiplier: usize) -> usize {
        let amt = self.cursor.move_by(m, multiplier);
        self.footer.at_cursor = self.total_at_cursor();
        self.splits.refresh_cursors(&self.cursor);
        amt
    }

    /// Sets the visible total (attempt or comparison, depending on `source`) to `time`.
    pub fn set_total(&mut self, time: PacedTime, source: aggregate::Source) {
        match source {
            aggregate::Source::Attempt => self.footer.total = time,
            aggregate::Source::Comparison => {
                let _ = self.footer.target.insert(time.time);
            }
        }
    }

    /// Tries to get the index of the split with short name `split`.
    #[must_use]
    pub fn index_of_split(&self, split: short::Name) -> Option<usize> {
        self.splits.index_of(split)
    }

    /// Tries to get the split at index `index` in the split set.
    #[must_use]
    pub fn split_at_index(&self, index: usize) -> Option<&self::Split> {
        self.splits.at_index(index)
    }

    /// Recalculates the state's footer totals.
    ///
    /// This generally needs to be done if the cursor has moved, or the split
    /// times have changed.
    pub fn refresh_footer_totals(&mut self) {
        self.footer.at_cursor = self.total_at_cursor();
    }

    /// Sets the editor at `index` to `editor`.
    pub fn set_editor(&mut self, index: usize, editor: Option<&super::Editor>) {
        self.splits.set_editor(index, editor);
    }

    /// Gets the total up to and excluding the current cursor position.
    fn total_at_cursor(&mut self) -> PacedTime {
        self.cursor_position()
            .checked_sub(1)
            .and_then(|c| self.splits.at_index(c))
            .map(Split::paced_cumulative)
            .unwrap_or_default()
    }

    /// Handles an event observation.
    pub fn handle_event(&mut self, ev: attempt::observer::Event) {
        use attempt::observer::Event;
        match ev {
            Event::Total(time, source) => self.set_total(time, source),
            Event::NumSplits(count) => self.set_split_count(count),
            Event::Reset(_) => self.reset(),
            Event::Attempt(a) => self.attempt = a,
            Event::GameCategory(gc) => self.game_category = gc,
            Event::Split(short, ev) => {
                self.handle_split_event(short, ev);
            }
        }
    }

    fn set_split_count(&mut self, count: usize) {
        self.splits.set_split_count(count);
        self.cursor.resize(count.saturating_sub(1));

        // The resize may have changed the splits' relative positions, so recalculate them.
        self.splits.refresh_cursors(&self.cursor);
    }

    /// Handles an observation for the split with the given shortname.
    fn handle_split_event(&mut self, short: short::Name, ev: attempt::observer::split::Event) {
        self.splits.handle_event(short, ev);
        // The changes to this split could have changed the overall and
        // up-to-cursor totals.
        self.refresh_footer_totals();

        // TODO(@MattWindsor91): open editor
    }
}

/// Presenter state about an editor.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Editor {
    /// The current field being edited, if any.
    pub field: Option<time::Position>,
    /// The current hours string.
    ///
    /// This is not actually exposed anywhere yet, but exists to simplify the index impl.
    pub hours: String,
    /// The current minutes string.
    pub mins: String,
    /// The current seconds string.
    pub secs: String,
    /// The current milliseconds string.
    pub msecs: String,
}

impl Editor {
    /// Gets a readout of the field at position `pos`.
    #[must_use]
    pub fn field(&self, pos: time::Position) -> &str {
        &*self[pos]
    }
}

impl Index<time::Position> for Editor {
    type Output = String;

    // TODO(@MattWindsor91): this returns &String, which is somewhat odd.
    fn index(&self, index: time::Position) -> &Self::Output {
        match index {
            time::Position::Hours => &self.hours,
            time::Position::Minutes => &self.mins,
            time::Position::Seconds => &self.secs,
            time::Position::Milliseconds => &self.msecs,
        }
    }
}

impl IndexMut<time::Position> for Editor {
    fn index_mut(&mut self, index: time::Position) -> &mut Self::Output {
        match index {
            time::Position::Hours => &mut self.hours,
            time::Position::Minutes => &mut self.mins,
            time::Position::Seconds => &mut self.secs,
            time::Position::Milliseconds => &mut self.msecs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that resetting the run clears the total.
    #[test]
    fn test_reset_clears_total() {
        let mut state = State::default();
        state.footer.total.time = crate::model::Time::seconds(1337).expect("shouldn't overflow");
        state.reset();
        assert!(state.footer.total.time.is_zero())
    }
}
