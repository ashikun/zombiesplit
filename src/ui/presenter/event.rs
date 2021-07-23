//! Events understood by the user interface.
use crate::model::time::position;

/// A high-level event.
///
/// The semantics of events depends on the current editing mode.
#[non_exhaustive]
pub enum Event {
    /// Undo something (the exact thing depends on the mode).
    Undo,
    /// Delete something completely (the exact thing depends on the mode).
    Delete,
    /// Commits whatever action is currently pending without moving the cursor.
    Commit,
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
