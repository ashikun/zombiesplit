//! Contains the split editor UI component.

use std::fmt::{Display, Formatter};

use super::{
    super::{
        cursor::{self, Cursor},
        State,
    },
    event::{Edit, Modal},
    nav::Nav,
    EventResult, Mode,
};
use crate::model::{attempt, time};

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
    fn on_entry(&mut self, state: &mut State) {
        // The cursor _should_ have been set by the preceding [Nav].
        state.set_editor(Some(self));
    }

    fn on_event(&mut self, ctx: super::EventContext) -> EventResult {
        let result = match ctx.event {
            Modal::Undo => self.undo(),
            Modal::Delete => self.delete(),
            Modal::Edit(d) => self.edit(d),
            Modal::EnterField(f) => self.enter_field(f),
            Modal::Cursor(c) => self.move_cursor(c),
            _ => EventResult::Handled,
        };
        // TODO(@MattWindsor91): this is suboptimal; we should only modify the
        // specific parts changed by the event.
        ctx.state.set_editor(Some(self));
        result
    }

    fn on_exit(&mut self, state: &mut State) -> Option<attempt::Action> {
        self.commit_field();
        state.set_editor(None);
        Some(attempt::Action::Push(
            self.cur.position(),
            std::mem::take(&mut self.time),
        ))
    }
}

impl Editor {
    /// Constructs a new editor at the given cursor, on the given field if any.
    #[must_use]
    pub fn new(cur: Cursor, field: Option<time::Position>) -> Self {
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
    pub fn enter_field(&mut self, field: time::Position) -> EventResult {
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
    position: time::Position,
    /// The current string.
    string: String,
}

impl Field {
    /// Creates a new editor for position `position`.
    #[must_use]
    pub fn new(position: time::Position) -> Self {
        Self {
            position,
            string: String::with_capacity(max_digits(position)),
        }
    }

    /// Gets this editor's position.
    #[must_use]
    pub fn position(&self) -> time::Position {
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
        time[self.position] = time::Field::parse(self.position, &self.string)?;
        Ok(())
    }

    fn max_digits(&self) -> usize {
        max_digits(self.position)
    }
}

fn max_digits(position: time::Position) -> usize {
    // TODO(@MattWindsor91): this should depend on the user format information
    match position {
        time::Position::Hours => 0, // for now
        time::Position::Minutes | time::Position::Seconds => 2,
        time::Position::Milliseconds => 3,
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:_<width$}", self.string, width = self.max_digits())
    }
}
