//! Modal events and ways to handle them.
use super::{
    super::{
        super::super::model::{session::action, timing::time},
        state::cursor::Motion,
    },
    Mode,
};

/// Context passed to `on_event` in a presenter mode.
#[derive(Debug)]
pub struct Context<'p> {
    /// The event being handled.
    pub event: Event,
    /// The visual state, which may need to be modified to reflect the event.
    pub state: &'p mut super::State,
}

/// Enum of results of handling an event in a mode.
pub enum Outcome {
    /// The event was handled internally.
    Handled,
    /// The event raised an action to be applied to the attempt model.
    Action(action::Action),
    /// The event caused a transition to another mode.
    Transition(Box<dyn Mode>),
}

impl Outcome {
    /// Shorthand for creating a transition.
    #[must_use]
    pub fn transition(to: impl Mode + 'static) -> Self {
        Self::Transition(Box::new(to))
    }

    /// Shorthand for creating a pop.
    #[must_use]
    pub fn pop(index: usize, ty: action::Pop) -> Self {
        Self::Action(action::Action::Pop(index, ty))
    }
}

/// A mode-specific event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// Undo something (the exact thing depends on the mode).
    Undo,
    /// Delete something completely (the exact thing depends on the mode).
    Delete,
    /// Commits whatever action is currently pending without moving the cursor.
    Commit,
    /// Perform an event on the currently open editor.
    Edit(Edit),
    /// Start editing a field at a particular position.
    EnterField(time::Position),
    /// Move the cursor.
    Cursor(Motion),
    /// Some sort of binary decision has been made.
    Decision(bool),
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
