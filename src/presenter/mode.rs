//! The Mode trait and associated functionality.

use crate::model::Session;

use super::{cursor::Cursor, editor::Editor, event::Event};

/// Trait for presenter modes.
///
/// zombiesplit is heavily modal, so most of the current presenter state
/// depends on the current mode.
pub trait Mode {
    /// Handles the given event according to the mode.
    ///
    /// The mode can modify the model in-place if needed, but `commit` will get
    /// called as the mode is transitioning out, and any modifications can be
    /// batched until then.
    ///
    /// Note that the presenter also handles some events at the global
    /// level.
    fn handle_event(&mut self, _e: &Event, _session: &mut Session) -> EventResult {
        EventResult::NotHandled
    }

    /// Commits any outstanding changes the mode needs to do to the model.
    fn commit(&mut self, _session: &mut Session) {}

    /// If this mode has a cursor, retrieves it.
    fn cursor(&self) -> Option<&Cursor> {
        None
    }

    /// If this mode has an editor, immutably borrows it.
    fn editor(&self) -> Option<&Editor> {
        None
    }

    /// Is zombiesplit running while this mode is active?
    fn is_running(&self) -> bool {
        true
    }
}

/// Mode for when there is no run active.
pub struct Inactive;

impl Mode for Inactive {}

/// Mode for when we are quitting.
pub struct Quitting;

impl Mode for Quitting {
    fn is_running(&self) -> bool {
        false
    }
}

/// Enum of results of handling an event in a mode.
pub enum EventResult {
    /// The event was not handled.
    NotHandled,
    /// The event was handled and the mode state should be considered dirty.
    Handled,
    /// The event caused a transition to another mode.
    Transition(Box<dyn Mode>),
}

impl EventResult {
    /// Creates an event result from a 'was handled' boolean `handled`.
    #[must_use]
    pub fn from_handled(handled: bool) -> Self {
        if handled {
            Self::Handled
        } else {
            Self::NotHandled
        }
    }

    /// Shorthand for creating a transition.
    #[must_use]
    pub fn transition(to: impl Mode + 'static) -> Self {
        Self::Transition(Box::new(to))
    }
}
