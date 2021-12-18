/*! The presenter's internal state.

This is populated from the model every time the presenter observes some kind
of change on the model.
*/

pub mod footer;
pub mod split;

use std::fmt::Display;
use std::ops::Index;

use crate::model::{
    aggregate, attempt, comparison::pace::PacedTime, game::category, short, time::position,
};

use super::cursor::SplitPosition;

pub use footer::Footer;
pub use split::Split;

/// The presenter's representation of the model.
#[derive(Debug, Default)]
pub struct State {
    pub cursor_pos: Option<usize>,

    /// The current attempt information.
    pub attempt: category::AttemptInfo,
    /// Information about the game and category being played.
    pub game_category: category::Info,

    // TODO(@MattWindsor91): this has a lot of invariants that need to be
    // maintained, so we should hide these next two fields or move the
    // short-position map elsewhere.
    /// Bidirectional map from split shortnames to their locations on the UI.
    pub short_map: short::Map<usize>,

    /// Split states.
    pub splits: Vec<Split>,

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
        for split in &mut self.splits {
            split.reset();
        }
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
        self.refresh_split_cursors();
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

    fn refresh_split_cursors(&mut self) {
        let c = self.cursor_pos;
        for (i, s) in &mut self.splits.iter_mut().enumerate() {
            s.position = SplitPosition::new(i, c);
        }
    }

    /// Sets the editor at the current cursor to `editor`.
    pub fn set_editor(&mut self, editor: Option<&super::Editor>) {
        let c = self.cursor_pos;
        // TODO(@MattWindsor91): this is a bit of a hack.
        for (i, s) in &mut self.splits.iter_mut().enumerate() {
            s.set_editor(editor.filter(|_| Some(i) == c));
        }
    }

    /// Gets the total up to and excluding the current cursor position.
    fn total_at_cursor(&mut self) -> PacedTime {
        self.cursor_pos
            .and_then(|x| x.checked_sub(1))
            .map(|c| self.splits[c].paced_cumulative())
            .unwrap_or_default()
    }

    /// Inserts a split into the split list with the given display name.
    pub fn add_split<T: Into<short::Name>, N: Display>(&mut self, short: T, name: N) {
        let short = short.into();
        let name = name.to_string();

        let split = Split {
            name,
            ..Split::default()
        };
        self.splits.push(split);
        self.short_map.insert(short, self.splits.len() - 1);
    }

    /// Handles an observation for the split with the given shortname.
    pub fn handle_split_event(&mut self, short: short::Name, evt: attempt::observer::split::Event) {
        if let Some(ref mut s) = self
            .short_map
            .get(&short)
            .copied()
            .and_then(|x| self.splits.get_mut(x))
        {
            s.handle_event(evt);
            // The changes to this split could have changed the overall and
            // up-to-cursor totals.
            self.refresh_footer_totals();

            // TODO(@MattWindsor91): open editor
        }
    }

    /// Gets the number of splits that the presenter is tracking.
    ///
    /// ```
    /// use zombiesplit::ui::presenter::state;
    ///
    /// let mut s = state::State::default();
    /// assert_eq!(0, s.num_splits());
    ///
    /// s.add_split("pp1", "Palmtree Panic 1");
    /// s.add_split("sp1", "Special Stage 1");
    /// s.add_split("pp2", "Palmtree Panic 2");
    /// assert_eq!(3, s.num_splits());
    /// ```
    #[must_use]
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }

    /// Gets the relative position of the split at `usize` relative to the
    /// cursor last logged in the split state.
    #[must_use]
    pub fn split_position(&self, split_pos: usize) -> SplitPosition {
        SplitPosition::new(split_pos, self.cursor_pos)
    }
}

/// Presenter state about an editor.
#[derive(Debug, Default, Clone)]
pub struct Editor {
    /// The current field being edited, if any.
    pub field: Option<position::Index>,
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
    /// Gets a readout of the field at position `name`.
    #[must_use]
    pub fn field(&self, name: position::Index) -> &str {
        match name {
            position::Index::Minutes => &self.mins,
            position::Index::Seconds => &self.secs,
            position::Index::Milliseconds => &self.msecs,
            position::Index::Hours => &self.hours,
        }
    }
}

impl Index<position::Index> for Editor {
    type Output = String;

    // TODO(@MattWindsor91): this is hacky.
    fn index(&self, index: position::Index) -> &Self::Output {
        match index {
            position::Index::Hours => &self.hours,
            position::Index::Minutes => &self.mins,
            position::Index::Seconds => &self.secs,
            position::Index::Milliseconds => &self.msecs,
        }
    }
}
