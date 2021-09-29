//! The Mode trait and associated functionality.

pub mod editor;
pub mod nav;

use crate::model::attempt::Session;

pub use editor::Editor;
pub use nav::Nav;

use super::{cursor::Cursor, event};

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
    fn handle_event(&mut self, _e: &event::Modal, _session: &mut Session) -> EventResult {
        EventResult::Handled
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
    /// The event was handled internally.
    Handled,
    /// The event has been interpreted as a change to the attempt state, and
    /// should be handled by the global event handler.
    Expanded(event::Attempt),
    /// The event caused a transition to another mode.
    Transition(Box<dyn Mode>),
}

impl EventResult {
    /// Shorthand for creating a transition.
    #[must_use]
    pub fn transition(to: impl Mode + 'static) -> Self {
        Self::Transition(Box::new(to))
    }
}
