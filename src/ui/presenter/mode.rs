//! The Mode trait and associated functionality.

pub mod editor;
pub mod nav;

pub use editor::Editor;
pub use nav::Nav;
use std::fmt::{Display, Formatter};

use super::{event, State};
use crate::model::session::action;

/// Trait for presenter modes.
///
/// zombiesplit is heavily modal, so most of the current presenter state
/// depends on the current mode.
///
/// Modes can:
///
/// - interpret a certain subset of UI events, turning them into events on the
///   model or transitions to other modes;
/// - modify the presenter's visual state;
/// - retain their own state, such as a split editor or a cursor.
///
/// They can also be `Display`ed; this should show a condensed form of the mode to fit within eg.
/// a status bar.
pub trait Mode: Display {
    /// Called when the mode has been swapped in.
    ///
    /// The [Mode] can perform any initialisation on the visual `state` here.
    fn on_entry(&mut self, state: &mut State);

    /// Handles the mode-specific event given in `ctx`.
    ///
    /// The mode also receives, in `ctx`, the ability to modify both the view
    /// state being displayed in the UI and parts of the downstream.  Note
    /// however that `commit` will get when this mode transitions out, and any
    /// modifications can be batched until then.
    ///
    /// Note that the presenter also handles some events at the global level.
    fn on_event(&mut self, ctx: EventContext) -> EventResult;

    /// Called when the mode is about to be swapped out.
    ///
    /// The [Mode] can perform any last-minute adjustments to the visual
    /// `state`, and optionally return a follow-on [Action] representing the
    /// application of this mode's efforts to the model.
    fn on_exit(&mut self, state: &mut State) -> Option<action::Action>;

    /// Is zombiesplit running while this mode is active?
    fn is_running(&self) -> bool {
        true
    }
}

/// Mode for when we are quitting.
pub struct Quitting;

impl Display for Quitting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("quit")
    }
}

impl Mode for Quitting {
    fn on_entry(&mut self, _state: &mut State) {}

    fn on_event(&mut self, _ctx: EventContext) -> EventResult {
        EventResult::Handled
    }

    fn on_exit(&mut self, _state: &mut State) -> Option<action::Action> {
        unreachable!("should not be able to exit out of the Quitting state")
    }

    fn is_running(&self) -> bool {
        false
    }
}

/// Context passed to `on_event` in a presenter mode.
#[derive(Debug)]
pub struct EventContext<'p> {
    /// The event being handled.
    pub event: event::Modal,
    /// The visual state, which may need to be modified to reflect the event.
    pub state: &'p mut super::State,
}

/// Enum of results of handling an event in a mode.
pub enum EventResult {
    /// The event was handled internally.
    Handled,
    /// The event raised an action to be applied to the attempt model.
    Action(action::Action),
    /// The event caused a transition to another mode.
    Transition(Box<dyn Mode>),
}

impl EventResult {
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
