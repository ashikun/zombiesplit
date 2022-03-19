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

/// The result of handling an event in a mode.
#[derive(Default)]
pub struct Outcome {
    /// Any actions to send to the session.
    pub actions: Vec<action::Action>,
    /// Any new mode to transition into.
    pub next_mode: Option<Box<dyn Mode>>,
}

impl Outcome {
    /// Shorthand for creating a transition.
    #[must_use]
    pub fn transition(to: impl Mode + 'static) -> Self {
        Self::boxed_transition(Box::new(to))
    }

    /// Shorthand for creating a boxed transition.
    #[must_use]
    pub fn boxed_transition(to: Box<dyn Mode>) -> Self {
        Outcome {
            actions: vec![],
            next_mode: Some(to),
        }
    }

    /// Shorthand for creating a single action.
    #[must_use]
    pub fn action(a: action::Action) -> Self {
        Outcome {
            actions: vec![a],
            next_mode: None,
        }
    }

    /// Shorthand for creating a pop.
    #[must_use]
    pub fn pop(index: usize, ty: action::Pop) -> Self {
        Self::action(action::Action::Pop(index, ty))
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
