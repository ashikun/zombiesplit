//! Contains the split editor UI component.

use std::fmt::{Display, Formatter};

use super::{
    event::{Edit, Event},
    mode::{EventResult, Mode},
};
use crate::model::{
    run::Run,
    time::{self, position},
};

/// A split editor.
pub struct Editor {
    pub cursor: usize,

    /// The time being edited.
    pub time: time::Time,

    /// The current field editor.
    pub field: Option<Field>,
}

impl Mode for Editor {
    fn handle_event(&mut self, e: &Event) -> EventResult {
        match e {
            Event::Edit(d) => self.edit(d),
            Event::EnterField(f) => self.enter_field(*f),
            _ => EventResult::NotHandled,
        }
    }

    fn commit(&mut self, run: &mut Run) {
        self.commit_field();
        if let Some(ref mut s) = run.splits.get_mut(self.cursor) {
            s.times.push(self.time)
        }
    }

    fn cursor_pos(&self) -> Option<usize> {
        Some(self.cursor)
    }

    fn editor(&self) -> Option<&Editor> {
        Some(&self)
    }
}

impl Editor {
    /// Constructs a new editor at the given cursor, on the given field if any.
    #[must_use]
    pub fn new(cursor: usize, field: Option<position::Name>) -> Self {
        Self {
            cursor,
            time: time::Time::default(),
            field: field.map(Field::new),
        }
    }

    /// Enters the named field, committing any edits on any current field.
    #[must_use]
    pub fn enter_field(&mut self, field: position::Name) -> EventResult {
        self.commit_field();
        self.field = Some(Field::new(field));
        EventResult::Handled
    }

    fn edit(&mut self, e: &Edit) -> EventResult {
        if self.field.as_mut().map_or(false, |f| f.edit(e)) {
            EventResult::Handled
        } else {
            EventResult::NotHandled
        }
    }

    pub fn commit_field(&mut self) {
        if let Some(ref f) = self.field {
            // TODO(@MattWindsor91): handle error properly.
            let _ = f.commit(&mut self.time);
        }
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

    pub fn edit(&mut self, e: &Edit) -> bool {
        match e {
            Edit::Add(x) => self.add(*x),
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
