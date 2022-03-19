//! Events understood by the user interface.

use super::{super::super::model::session, mode};

/// A high-level event.
///
/// The semantics of events depends on the current editing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    /// An event that should be interpreted by the current mode.
    Modal(mode::Event),
    /// An event that translates directly into an action on the current attempt.
    /// These are handled globally.
    Action(session::Action),
    /// A request to quit the user interface.
    /// If `force` is false, the presenter will ask the user first.
    Quit { force: bool },
}

impl Event {
    // Mappings from UI events to presenter events are in the `view` crate.

    /// Shorthand for producing a field event.
    #[must_use]
    pub const fn digit(digit: u8) -> Self {
        Self::Modal(mode::Event::Edit(mode::event::Edit::Add(digit)))
    }

    /// Shorthand for producing a modal decision event.
    #[must_use]
    pub const fn decision(value: bool) -> Self {
        Self::Modal(mode::Event::Decision(value))
    }

    /// Shorthand for producing a cursor motion event.
    #[must_use]
    pub const fn motion(m: super::state::cursor::Motion) -> Event {
        Self::Modal(mode::Event::Cursor(m))
    }
}
