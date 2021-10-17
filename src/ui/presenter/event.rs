//! Events understood by the user interface.
use crate::model::{attempt::Action, time::position};

/// A high-level event.
///
/// The semantics of events depends on the current editing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// An event that should be interpreted by the current mode.
    Modal(Modal),
    /// An event that translates directly into an action on the current attempt.
    /// These are handled globally.
    Action(Action),
    /// A request to quit the user interface.
    Quit,
}

impl Event {
    // Mappings from UI events to presenter events are in the `view` crate.

    /// Shorthand for producing a field event.
    #[must_use]
    pub fn digit(digit: u8) -> Self {
        Self::Modal(Modal::Edit(Edit::Add(digit)))
    }
}

/// A mode-specific event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Modal {
    /// Undo something (the exact thing depends on the mode).
    Undo,
    /// Delete something completely (the exact thing depends on the mode).
    Delete,
    /// Commits whatever action is currently pending without moving the cursor.
    Commit,
    /// Perform an event on the currently open editor.
    Edit(Edit),
    /// Start editing a field at a particular position.
    EnterField(position::Name),
    /// Move the cursor.
    Cursor(super::cursor::Motion),
}

/// An edit event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Edit {
    /// Add the given digit to the current editor.
    Add(u8),
    /// Remove the last item (for instance, a digit) from the current editor.
    Remove,
}
