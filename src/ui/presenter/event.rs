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
    Quit,
}

impl Event {
    // Mappings from UI events to presenter events are in the `view` crate.

    /// Shorthand for producing a field event.
    #[must_use]
    pub fn digit(digit: u8) -> Self {
        Self::Modal(mode::Event::Edit(mode::event::Edit::Add(digit)))
    }

    /// Shorthand for producing a modal decision event.
    #[must_use]
    pub fn decision(value: bool) -> Self {
        Self::Modal(mode::Event::Decision(value))
    }
}

/// Trait for things that can produce events to be passed to the presenter.
///
/// The unusual structure of this trait, where the presenter target is made explicit, serves to
/// avoid the use of a boxed iterator.
pub trait Pump<H> {
    /// Pumps this pump's events.
    ///
    /// The implementation should call `send_to.handle_event` for each event detected.
    fn pump<'a>(&'a mut self, send_to: &'a mut super::Presenter<H>);
}
