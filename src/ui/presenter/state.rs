/*! The presenter's internal state.

This is populated from the model every time the presenter observes some kind
of change on the model.
*/

pub mod footer;
pub mod split;

use std::ops::{Index, IndexMut};

use crate::model::{aggregate, attempt, comparison::pace::PacedTime, game::category, short, time};

use super::cursor::SplitPosition;

pub use footer::Footer;
pub use split::Split;

/// The presenter's representation of the model.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct State {
    pub cursor_pos: Option<usize>,

    /// The current attempt information.
    pub attempt: category::AttemptInfo,
    /// Information about the game and category being played.
    pub game_category: category::Info,

    /// State for the splits being displayed in the UI.
    pub splits: split::Set,

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
        self.splits.reset();
        self.footer.total = PacedTime::default();
    }

    /// Disables every optional visual element (cursor, editor, etc).
    pub fn disable_everything(&mut self) {
        self.set_cursor(None);
        self.set_editor(None);
    }

    /// Sets the visible cursor position to `cursor_at`.
    pub fn set_cursor(&mut self, cursor_at: Option<usize>) {
        self.cursor_pos = cursor_at;
        self.footer.at_cursor = self.total_at_cursor();
        self.splits.refresh_cursors(cursor_at);
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

    /// Recalculates the state's footer totals.
    ///
    /// This generally needs to be done if the cursor has moved, or the split
    /// times have changed.
    pub fn refresh_footer_totals(&mut self) {
        self.footer.at_cursor = self.total_at_cursor();
    }

    /// Sets the editor at the current cursor to `editor`.
    pub fn set_editor(&mut self, editor: Option<&super::Editor>) {
        self.splits.set_editor(self.cursor_pos, editor);
    }

    /// Gets the total up to and excluding the current cursor position.
    fn total_at_cursor(&mut self) -> PacedTime {
        self.cursor_pos
            .and_then(|x| x.checked_sub(1))
            .and_then(|c| self.splits.at_index(c))
            .map(Split::paced_cumulative)
            .unwrap_or_default()
    }

    /// Handles an observation for the split with the given shortname.
    pub fn handle_split_event(&mut self, short: short::Name, evt: attempt::observer::split::Event) {
        self.splits.handle_event(short, evt);
        // The changes to this split could have changed the overall and
        // up-to-cursor totals.
        self.refresh_footer_totals();

        // TODO(@MattWindsor91): open editor
    }

    /// Gets the relative position of the split at `usize` relative to the
    /// cursor last logged in the split state.
    #[must_use]
    pub fn split_position(&self, split_pos: usize) -> SplitPosition {
        SplitPosition::new(split_pos, self.cursor_pos)
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
