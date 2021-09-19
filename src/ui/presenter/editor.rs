//! Contains the split editor UI component.

use std::fmt::{Display, Formatter};

use super::{
    cursor::{self, Cursor},
    event::{Edit, Modal},
    mode::{EventResult, Mode},
    nav::Nav,
};
use crate::model::{
    attempt::Session,
    time::{self, position},
};

/// A split editor.
pub struct Editor {
    /// The cursor, used to track the current position for later navigation.
    pub cur: Cursor,

    /// The time being edited.
    pub time: time::Time,

    /// The current field editor.
    pub field: Option<Field>,
}

impl Mode for Editor {
    fn handle_event(&mut self, e: &Modal, _: &mut Session) -> EventResult {
        match e {
            Modal::Undo => self.undo(),
            Modal::Delete => self.delete(),
            Modal::Edit(d) => self.edit(*d),
            Modal::EnterField(f) => self.enter_field(*f),
            Modal::Cursor(c) => self.move_cursor(*c),
            _ => EventResult::Handled,
        }
    }

    fn commit(&mut self, session: &mut Session) {
        self.commit_field();
        session.push_to(self.cur.position(), std::mem::take(&mut self.time));
    }

    fn cursor(&self) -> Option<&Cursor> {
        Some(&self.cur)
    }

    fn editor(&self) -> Option<&Editor> {
        Some(self)
    }
}

impl Editor {
    /// Constructs a new editor at the given cursor, on the given field if any.
    #[must_use]
    pub fn new(cur: Cursor, field: Option<position::Name>) -> Self {
        Self {
            cur,
            time: time::Time::default(),
            field: field.map(Field::new),
        }
    }

    /// Constructs a new editor with the given cursor and time, and with no
    /// field open.
    #[must_use]
    pub fn with_time(cur: Cursor, time: time::Time) -> Self {
        Self {
            cur,
            time,
            field: None,
        }
    }

    /// Enters the named field, committing any edits on any current field.
    #[must_use]
    pub fn enter_field(&mut self, field: position::Name) -> EventResult {
        self.commit_field();
        self.field = Some(Field::new(field));
        EventResult::Handled
    }

    fn edit(&mut self, e: Edit) -> EventResult {
        if let Some(f) = self.field.as_mut() {
            f.edit(e);
        }
        EventResult::Handled
    }

    fn undo(&mut self) -> EventResult {
        // Try clearing the field first, then, otherwise, clear the time.
        if self.field.take().is_none() {
            self.time = time::Time::default();
        }
        EventResult::Handled
    }

    fn delete(&mut self) -> EventResult {
        self.field = None;
        self.time = time::Time::default();
        Nav::transition(self.cur)
    }

    /// Commits the field currently being edited.
    pub fn commit_field(&mut self) {
        if let Some(ref f) = self.field.take() {
            // TODO(@MattWindsor91): handle error properly.
            let _ = f.commit(&mut self.time);
        }
    }

    /// Performs the given cursor motion.
    #[must_use]
    pub fn move_cursor(&mut self, motion: cursor::Motion) -> EventResult {
        // Need to copy the cursor, so that the editor commits to the
        // right location.
        let mut cur = self.cur;
        cur.move_by(motion, 1);
        Nav::transition(cur)
    }
}

/// A split field editor.
pub struct Field {
    /// The position being edited.
    position: position::Name,
    /// The current string.
    string: String,
}

impl Field {
    /// Creates a new editor for position `position`.
    #[must_use]
    pub fn new(position: position::Name) -> Self {
        Self {
            position,
            string: String::with_capacity(max_digits(position)),
        }
    }

    /// Gets this editor's position.
    #[must_use]
    pub fn position(&self) -> position::Name {
        self.position
    }

    pub fn edit(&mut self, e: Edit) -> bool {
        match e {
            Edit::Add(x) => self.add(x),
            Edit::Remove => self.remove(),
        }
    }

    /// Adds a digit to the editor.
    #[must_use]
    pub fn add(&mut self, digit: u8) -> bool {
        if self.max_digits() <= self.string.len() {
            false
        } else {
            self.string.push_str(&digit.to_string());
            true
        }
    }

    /// Removes a digit from the editor.
    #[must_use]
    pub fn remove(&mut self) -> bool {
        self.string.pop().is_some()
    }

    /// Commits this editor's changes to `time`.
    ///
    /// # Errors
    ///
    /// Fails if the string is not parseable for the particular field we're
    /// editing.
    pub fn commit(&self, time: &mut time::Time) -> time::error::Result<()> {
        time.set_field_str(self.position, &self.string)
    }

    fn max_digits(&self) -> usize {
        max_digits(self.position)
    }
}

fn max_digits(position: position::Name) -> usize {
    use position::Name;
    match position {
        Name::Hours => 0, // for now
        Name::Minutes | Name::Seconds => 2,
        Name::Milliseconds => 3,
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:_<width$}", self.string, width = self.max_digits())
    }
}
