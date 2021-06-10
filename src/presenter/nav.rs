//! The [Nav] struct and its implementations.

use super::{
    cursor::{self, Cursor},
    editor::Editor,
    event::Event,
    mode::{EventResult, Mode},
};
use crate::model::{split::Set, time::position, Session};

/// Mode for when we are navigating splits.
pub struct Nav {
    /// The cursor.
    cur: Cursor,
}

impl Mode for Nav {
    fn handle_event(&mut self, e: &Event, s: &mut Session) -> EventResult {
        match e {
            Event::Cursor(c) => self.move_cursor(*c),
            Event::EnterField(f) => self.enter_field(*f),
            Event::Undo => self.undo(s),
            Event::Delete => self.delete(s),
            _ => EventResult::NotHandled,
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
    fn undo(&mut self, s: &mut Session) -> EventResult {
        s.pop_from(self.cur.position())
            .map_or(EventResult::Handled, |time| {
                EventResult::transition(Editor::with_time(self.cur, time))
            })
    }

    /// Performs a delete on the current split, if any.
    fn delete(&mut self, s: &mut Session) -> EventResult {
        s.clear_at(self.cur.position());
        EventResult::Handled
    }

    /// Moves the state cursor according to `c`, if possible.
    fn move_cursor(&mut self, motion: cursor::Motion) -> EventResult {
        // TODO(@MattWindsor91): cursor multiplier
        EventResult::from_handled(self.cur.move_by(motion, 1) != 0)
    }

    /// Constructs an editor entering the given field.
    fn enter_field(&self, field: position::Name) -> EventResult {
        let editor = Editor::new(self.cur, Some(field));
        EventResult::transition(editor)
    }
}
