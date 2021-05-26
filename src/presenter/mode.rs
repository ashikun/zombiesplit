//! The Mode trait and associated functionality.

use crate::model::time::position;

use super::{
    editor::Editor,
    event::{Cursor, Event},
};

/// Trait for presenter modes.
///
/// zombiesplit is heavily modal, so most of the current presenter state
/// depends on the current mode.
pub trait Mode {
    /// Handles the given event according to the mode.
    ///
    /// Note that the presenter also handles some events at the global
    /// level.
    fn handle_event(&mut self, _e: &Event) -> EventResult {
        EventResult::NotHandled
    }

    /// Commits this mode's changes to the model.
    fn commit(&mut self, _run: &mut crate::model::run::Run) {}

    /// If this mode has a cursor, retrieves its current position.
    fn cursor_pos(&self) -> Option<usize> {
        None
    }

    /// If this mode has an editor, immutably borrows it.
    fn editor(&self) -> Option<&Editor> {
        None
    }

    /// Is zombiesplit running while this mode is active?
    fn is_running(&self) -> bool {
        true
    }
}

/// Mode for when there is no run active.
pub struct Inactive;

impl Mode for Inactive {}

/// Mode for when we are navigating splits.
pub struct Nav {
    cursor: usize,
    /// The maximum for the navigation.
    max: usize,
}

impl Mode for Nav {
    fn handle_event(&mut self, e: &Event) -> EventResult {
        match e {
            Event::Cursor(c) => self.move_cursor(*c),
            Event::EnterField(f) => self.enter_field(*f),
            _ => EventResult::NotHandled,
        }
    }

    fn cursor_pos(&self) -> Option<usize> {
        Some(self.cursor)
    }
}

impl Nav {
    /// Creates a new nav mode.
    #[must_use]
    pub fn new(max: usize) -> Self {
        Self { cursor: 0, max }
    }

    /// Moves the state cursor according to `c`, if possible.
    fn move_cursor(&mut self, c: Cursor) -> EventResult {
        match c {
            Cursor::Up => self.move_cursor_up(),
            Cursor::Down => self.move_cursor_down(),
        }
    }

    /// Try to move the cursor up.
    fn move_cursor_up(&mut self) -> EventResult {
        if self.cursor == 0 {
            EventResult::NotHandled
        } else {
            self.cursor -= 1;
            EventResult::Handled
        }
    }

    /// Try to move the cursor down.
    fn move_cursor_down(&mut self) -> EventResult {
        if self.cursor == self.max {
            EventResult::NotHandled
        } else {
            self.cursor += 1;
            EventResult::Handled
        }
    }

    /// Constructs an editor entering the given field.
    fn enter_field(&self, field: position::Name) -> EventResult {
        let editor = Box::new(Editor::new(self.cursor, Some(field)));
        EventResult::Transition(editor)
    }
}

/// Mode for when we are quitting.
pub struct Quitting;

impl Mode for Quitting {
    fn is_running(&self) -> bool {
        false
    }
}

/// Enum of results of handling an event in a mode.
pub enum EventResult {
    /// The event was not handled.
    NotHandled,
    /// The event was handled and the mode state should be considered dirty.
    Handled,
    /// The event caused a transition to another mode.
    Transition(Box<dyn Mode>),
}
