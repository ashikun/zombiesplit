//! Contains the split editor UI component.

use std::fmt::{Display, Formatter};

use crate::model::time::position;

/// A split field editor.
pub struct Editor {
    /// The position being edited.
    position: position::Name,
    /// The current string.
    string: String,
}

impl Editor {
    /// Creates a new editor for position `position`.
    pub fn new(position: position::Name) -> Self {
        Self {
            position,
            string: String::with_capacity(max_digits(position)),
        }
    }

    /// Gets this editor's position.
    pub fn position(&self) -> position::Name {
        self.position
    }

    /// Adds a digit to the editor.
    pub fn add(&mut self, digit: u8) -> bool {
        if self.max_digits() <= self.string.len() {
            false
        } else {
            self.string.push_str(&digit.to_string());
            true
        }
    }

    /// Removes a digit from the editor.
    pub fn remove(&mut self) -> bool {
        self.string.pop().is_some()
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

impl Display for Editor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:_<width$}", self.string, width = self.max_digits())
    }
}
