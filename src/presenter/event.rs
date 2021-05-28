//! Events understood by the user interface.
use crate::model::time::position;

/// High-level event, translated from a SDL event.
#[non_exhaustive]
pub enum Event {
    /// Undo something (the exact thing depends on the mode).
    ///
    /// For instance:
    /// - Nav -> removes one time for a split and returns it to the editor.
    /// - Editor -> removes the field, if any.
    Undo,
    /// Delete something completely (the exact thing depends on the mode).
    ///
    /// For instance:
    /// - Nav -> removes all times for a split.
    /// - Editor -> removes the entire split being edited and returns to Nav.
    Delete,
    /// Start editing a field at a particular position.
    EnterField(position::Name),
    /// Perform an event on the currently open editor.
    Edit(Edit),
    /// Start a new run.
    NewRun,
    /// Move the cursor.
    Cursor(super::cursor::Motion),
    /// Quit the program.
    Quit,
}

impl Event {
    // Mappings from UI events to presenter events are in the `view` crate.

    /// Shorthand for producing a field event.
    #[must_use]
    pub fn digit(digit: u8) -> Self {
        Self::Edit(Edit::Add(digit))
    }
}

/// An edit event.
#[non_exhaustive]
pub enum Edit {
    /// Add the given digit to the current editor.
    Add(u8),
    /// Remove the last item (for instance, a digit from the current editor.
    Remove,
}
