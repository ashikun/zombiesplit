//! The [Nav] struct and its implementations.

use super::{
    cursor::{self, Cursor},
    editor::Editor,
    event::{self, Modal},
    mode::{EventResult, Mode},
};
use crate::model::{attempt::Session, time::position};

/// Mode for when we are navigating splits.
pub struct Nav {
    /// The cursor.
    cur: Cursor,
}

impl Mode for Nav {
    fn handle_event(&mut self, e: &Modal, s: &mut Session) -> EventResult {
        match e {
            Modal::Cursor(c) => self.move_cursor(*c),
            Modal::EnterField(f) => self.enter_field(*f),
            Modal::Undo => self.undo(),
            Modal::Delete => self.delete(s),
            _ => EventResult::Handled,
        }
    }

    fn cursor(&self) -> Option<&Cursor> {
        Some(&self.cur)
    }
}

impl Nav {
    /// Creates a new nav mode using a given cursor.
    #[must_use]
    pub fn new(cur: Cursor) -> Self {
        Self { cur }
    }

    /// Creates a transition to a navigation from the given cursor.
    #[must_use]
    pub fn transition(cur: Cursor) -> EventResult {
        EventResult::transition(Self::new(cur))
    }

    /// Performs an undo on the current split, if any.
    fn undo(&mut self) -> EventResult {
        EventResult::Expanded(event::Attempt::Pop(self.cur.position()))
    }

    /// Performs a delete on the current split, if any.
    fn delete(&mut self, s: &mut Session) -> EventResult {
        s.clear_at(self.cur.position());
        EventResult::Handled
    }

    /// Moves the state cursor according to `c`, if possible.
    fn move_cursor(&mut self, motion: cursor::Motion) -> EventResult {
        // TODO(@MattWindsor91): cursor multiplier
        self.cur.move_by(motion, 1);
        EventResult::Handled
    }

    /// Constructs an editor entering the given field.
    fn enter_field(&self, field: position::Name) -> EventResult {
        let editor = Editor::new(self.cur, Some(field));
        EventResult::transition(editor)
    }
}
