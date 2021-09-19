//! Events understood by the user interface.
use crate::model::{time::position, Time};

/// A high-level event.
///
/// The semantics of events depends on the current editing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// An event that should be interpreted by the current mode.
    Modal(Modal),
    /// An event that directly affects the attempt, regardless of mode.
    /// These are handled globally.
    Attempt(Attempt),
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

/// An event that manipulates the current event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Attempt {
    // TODO(@MattWindsor91): move these to a session API
    /// Start a new run.
    NewRun,
    /// Quit the attempt.
    Quit,
    /// Pushes a time to the split at the given position.
    Push(usize, Time),
    /// Pops a time from the split at the given position.
    Pop(usize),
    /// Erases the split at the given position.
    Clear(usize),
}
