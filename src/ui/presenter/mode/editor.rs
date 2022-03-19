//! Contains the split editor UI component.

use std::fmt::{Display, Formatter};

use super::{
    super::{
        super::super::model::{session, timing::time},
        state::{cursor, State},
    },
    event,
    nav::Nav,
    Mode,
};

/// A split editor.
pub struct Editor {
    /// The time being edited.
    pub time: time::Time,

    /// The current field editor.
    pub field: Option<Field>,

    /// The index of the split being edited.
    pub index: usize,
}

impl Display for Editor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(fld) = self.field.as_ref() {
            write!(f, "edit[{}]", fld.position)
        } else {
            f.write_str("edit")
        }
    }
}

impl Mode for Editor {
    fn on_entry(&mut self, state: &mut State) {
        state.set_editor(self.index, Some(self));
    }

    fn on_event(&mut self, ctx: event::Context) -> event::Outcome {
        let result = match ctx.event {
            event::Event::Undo => self.undo(),
            event::Event::Delete => self.delete(),
            event::Event::Edit(d) => self.edit(d),
            event::Event::EnterField(f) => self.enter_field(f),
            event::Event::Cursor(c) => move_cursor(c, ctx.state),
            _ => event::Outcome::default(),
        };
        // TODO(@MattWindsor91): this is suboptimal; we should only modify the
        // specific parts changed by the event.
        ctx.state.set_editor(self.index, Some(self));
        result
    }

    fn on_exit(&mut self, state: &mut State) -> Vec<session::Action> {
        self.commit_field();
        state.set_editor(self.index, None);
        vec![session::Action::Push(
            self.index,
            std::mem::take(&mut self.time),
        )]
    }
}

impl Editor {
    /// Constructs a new editor at the given index, on the given field if any.
    #[must_use]
    pub fn new(index: usize, field: Option<time::Position>) -> Self {
        Self {
            time: time::Time::default(),
            field: field.map(Field::new),
            index,
        }
    }

    /// Enters the named field, committing any edits on any current field.
    #[must_use]
    pub fn enter_field(&mut self, field: time::Position) -> event::Outcome {
        self.commit_field();
        self.field = Some(Field::new(field));
        event::Outcome::default()
    }

    fn edit(&mut self, e: event::Edit) -> event::Outcome {
        if let Some(f) = self.field.as_mut() {
            f.edit(e);
        }
        event::Outcome::default()
    }

    fn undo(&mut self) -> event::Outcome {
        // Try clearing the field first, then, otherwise, clear the time.
        if self.field.take().is_none() {
            self.time = time::Time::default();
        }
        event::Outcome::default()
    }

    fn delete(&mut self) -> event::Outcome {
        self.field = None;
        self.time = time::Time::default();
        Nav::transition()
    }

    /// Commits the field currently being edited.
    fn commit_field(&mut self) {
        if let Some(ref f) = self.field.take() {
            // TODO(@MattWindsor91): handle error properly.
            let _ = f.commit(&mut self.time);
        }
    }
}

/// Moves a cursor using the given motion, exiting the editor.
fn move_cursor(motion: cursor::Motion, state: &mut State) -> event::Outcome {
    state.move_cursor_by(motion, 1);
    Nav::transition()
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

    fn edit(&mut self, e: event::Edit) {
        match e {
            event::Edit::Add(x) => self.add(x),
            event::Edit::Remove => self.remove(),
        }
    }

    /// Adds a digit to the editor.
    fn add(&mut self, digit: u8) {
        if self.string.len() < self.max_digits() {
            self.string.push_str(&digit.to_string());
        }
    }

    /// Removes a digit from the editor.
    fn remove(&mut self) {
        let _ = self.string.pop();
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
