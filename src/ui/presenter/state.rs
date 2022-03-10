/*! The presenter's internal state.

This is populated from the model every time the presenter observes some kind
of change on the model.
*/

pub use editor::Editor;
pub use footer::Footer;
pub use split::Split;

use crate::model::{
    game::category,
    session, short,
    timing::{comparison::pace::PacedTime, time},
};

pub mod cursor;
pub mod editor;
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

    /// Stringified name of the current mode.
    pub mode: String,

    /// State for the splits being displayed in the UI.
    splits: split::Set,

    /// State for the footer widget.
    pub footer: Footer,
}

impl State {
    /// Creates a new client-side state from an initial server-side state dump.
    #[must_use]
    pub fn from_dump(dump: &session::State) -> Self {
        Self {
            cursor: cursor::Cursor::new(0, dump.run.splits.len() - 1),
            attempt: dump.run.info,
            game_category: dump.run.category.clone(),
            mode: "Welcome to zombiesplit!".to_string(),
            splits: split::Set::from_dump(dump),
            footer: footer::Footer::from_dump(dump),
        }
    }

    /// Makes the presenter state reflect a reset in the run.
    ///
    /// This clears the time count aggregate data for all splits.  It doesn't
    /// change the attempt information, as we expect there will be a separate
    /// observation for that.
    pub fn reset(&mut self, new_attempt: &category::AttemptInfo) {
        self.cursor.reset();
        self.splits.refresh_cursors(&self.cursor);
        self.splits.reset();
        self.footer.total = PacedTime::default();
        self.attempt = *new_attempt;
    }

    /// Gets the current position of the cursor.
    #[must_use]
    pub fn cursor_position(&self) -> usize {
        self.cursor.position()
    }

    /// Gets the total number of splits in the state.
    #[must_use]
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }

    /// Moves the cursor in the direction of `m` `multiplier` times.
    /// Returns the absolute amount by which the cursor moved.
    pub fn move_cursor_by(&mut self, m: cursor::Motion, multiplier: usize) -> usize {
        let amt = self.cursor.move_by(m, multiplier);
        self.footer.at_cursor = self.total_at_cursor();
        self.splits.refresh_cursors(&self.cursor);
        amt
    }

    /// Sets a total (eg attempt, comparison, sum-of-best).
    pub fn set_total(&mut self, ty: session::event::Total, time: Option<time::Time>) {
        match ty {
            session::event::Total::Attempt(pace) => {
                self.footer.total = PacedTime {
                    time: time.unwrap_or_default(),
                    pace,
                };
            }
            session::event::Total::Comparison(ty) => {
                self.footer.comparisons[ty] = time;
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

    // Tries to get the split at shortname `short` in the split set.
    #[must_use]
    pub fn split_at_short(&self, short: short::Name) -> Option<&self::Split> {
        self.splits.at_short(short)
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
    pub fn handle_event(&mut self, ev: &session::event::Event) {
        use session::event::Event;
        match ev {
            Event::Total(ty, time) => self.set_total(*ty, *time),
            Event::Reset(a) => self.reset(a),
            Event::Split(short, ev) => {
                self.handle_split_event(*short, ev);
            }
        }
    }

    /// Handles an observation for the split with the given shortname.
    fn handle_split_event(&mut self, short: short::Name, ev: &session::event::split::Split) {
        self.splits.handle_event(short, ev);
        // The changes to this split could have changed the overall and
        // up-to-cursor totals.
        self.refresh_footer_totals();

        // TODO(@MattWindsor91): open editor
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
        state.reset(&category::AttemptInfo::default());
        assert!(state.footer.total.time.is_zero());
    }
}
