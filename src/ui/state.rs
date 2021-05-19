//! Contains all of the state held by the user interface.

use super::event;
use crate::model::{run, time::position};

/// The state held by the user interface.
pub struct State {
    // TODO(@MattWindsor91): this is perhaps more of a presenter.

    /// The current split.
    pub cursor: usize,
    /// The current run.
    pub run: run::Run,
    /// The current action that the UI is taking.
    pub action: Action,
    /// Whether the state is dirty.
    pub is_dirty: bool,
}

impl State {
    /// Constructs a new initial state for a given run.
    pub fn new(run: run::Run) -> Self {
        Self {
            cursor: 0,
            run,
            action: Action::default(),
            is_dirty: false,
        }
    }

    /// Produces a vector of split references.
    pub fn splits(&self) -> Vec<SplitRef> {
        self.run
            .splits
            .iter()
            .enumerate()
            .map(|(index, split)| SplitRef {
                index,
                split,
                state: self,
            })
            .collect()
    }

    /// Gets whether the UI should be running.
    pub fn is_running(&self) -> bool {
        !matches!(self.action, Action::Quit)
    }

    /// Gets whether the UI is tracking an active run.
    pub fn is_on_run(&self) -> bool {
        matches!(self.action, Action::Nav | Action::Entering{..})
    }

    /// Handles an event.  Returns true if the event changed the state.
    pub fn handle_event(&mut self, e: &event::Event) {
        match e {
            event::Event::EnterField(name) => self.enter_field(*name),
            event::Event::NewRun => self.start_new_run(),
            event::Event::CursorDown => self.move_cursor_down(),
            event::Event::CursorUp => self.move_cursor_up(),
            event::Event::Quit => {
                self.action = Action::Quit;
            }
        }
    }

    /// Enters a field.
    fn enter_field(&mut self, field: position::Name) {
        self.action = Action::Entering{field, entry: String::with_capacity(3)};
        self.dirty()
    }

    /// Starts a new run, abandoning any previous run.
    fn start_new_run(&mut self) {
        // TODO(@MattWindsor91): actually reset the run here.
        self.action = Action::Nav;
        self.cursor = 0;
        self.dirty()
    }

    /// Moves the state cursor up, if possible.
    fn move_cursor_up(&mut self) {
        if self.can_move_cursor_up() {
            self.cursor -= 1;
            self.dirty()
        }
    }

    /// Works out whether the cursor can be moved up.
    fn can_move_cursor_up(&self) -> bool {
        self.is_on_run() && self.cursor != 0
    }

    /// Moves the state cursor down, if possible
    fn move_cursor_down(&mut self) {
        if self.can_move_cursor_down() {
            self.cursor += 1;
            self.dirty()
        }
    }

    /// Works out whether the cursor can be moved down.
    fn can_move_cursor_down(&self) -> bool {
        self.is_on_run() && self.cursor != self.run.splits.len() - 1
    }

    // Marks the UI as dirty.
    fn dirty(&mut self) {
        self.is_dirty = true
    }
}

/// The current action the user interface is performing.
pub enum Action {
    /// Run is inactive.
    Inactive,
    /// Currently navigating the splits.
    Nav,
    /// Currently entering a field in the active split.
    Entering{ field: crate::model::time::position::Name, entry: String },
    /// ZombieSplit is quitting.
    Quit,
}

/// The default [Action] is `Inactive`.
impl Default for Action {
    fn default() -> Self {
        Action::Inactive
    }
}

/// A split reference, containing position information the split.
#[derive(Copy, Clone)]
pub struct SplitRef<'a> {
    /// The index of the split reference.
    pub index: usize,
    /// A reference to the parent state.
    pub state: &'a State,
    /// The split data.
    pub split: &'a crate::model::run::Split,
}

impl<'a> SplitRef<'a> {
    /// Gets whether this split is currently active.
    pub fn position(&self) -> SplitPosition {
        if self.state.is_on_run() {
            match self.index.cmp(&self.state.cursor) {
                std::cmp::Ordering::Less => SplitPosition::Done,
                std::cmp::Ordering::Equal => SplitPosition::Cursor,
                std::cmp::Ordering::Greater => SplitPosition::Coming,
            }
        } else {
            SplitPosition::Coming
        }
    }
}

/// Relative positions of splits to cursors.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum SplitPosition {
    /// This split is before the cursor.
    Done,
    /// This split is on the cursor.
    Cursor,
    /// This split is after the cursor.
    Coming,
}

impl<'a> AsRef<crate::model::run::Split> for SplitRef<'a> {
    fn as_ref(&self) -> &crate::model::run::Split {
        self.split
    }
}
