//! Contains all of the state held by the user interface.

use super::event;
use crate::model::run;

pub struct State {
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
    pub fn new(run: run::Run) -> Self {
        Self {
            cursor: 0,
            run,
            action: Action::default(),
            is_dirty: false,
        }
    }

    /// Gets whether the UI should be running.
    pub fn is_running(&self) -> bool {
        !matches!(self.action, Action::Quit)
    }

    /// Handles an event.  Returns true if the event changed the state.
    pub fn handle_event(&mut self, e: &event::Event) {
        match e {
            event::Event::CursorDown => self.move_cursor_down(),
            event::Event::CursorUp => self.move_cursor_up(),
            event::Event::Quit => {
                self.action = Action::Quit;
            }
        }
    }

    /// Moves the state cursor up.
    pub fn move_cursor_up(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
            self.is_dirty = true
        }
    }

    /// Moves the state cursor down.  Returns true if the cursor moved successfully.
    pub fn move_cursor_down(&mut self) {
        if self.cursor != self.run.splits.len() - 1 {
            self.cursor += 1;
            self.is_dirty = true
        }
    }
}

/// The current action the user interface is performing.
pub enum Action {
    /// Run is inactive.
    Inactive,
    /// Currently navigating the splits.
    //Nav,
    /// Currently entering a field in the active split.
    //Entering{ field: time::Field, entry: String },
    /// ZombieSplit is quitting.
    Quit,
}

/// The default [Action] is `Inactive`.
impl Default for Action {
    fn default() -> Self {
        Action::Inactive
    }
}
