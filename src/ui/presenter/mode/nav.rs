//! The [Nav] struct and its implementations.

use crate::model::session::action;
use crate::model::timing::time;
use std::fmt::{Display, Formatter};

use super::{
    super::state::{cursor, State},
    editor::Editor,
    event::Modal,
    EventContext, EventResult, Mode,
};

/// Mode for when we are navigating splits.
pub struct Nav;

impl Display for Nav {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("normal")
    }
}

impl Mode for Nav {
    fn on_entry(&mut self, _state: &mut State) {}

    fn on_event(&mut self, EventContext { event, state, .. }: EventContext) -> EventResult {
        match event {
            Modal::Cursor(c) => move_cursor(c, state),
            Modal::EnterField(f) => enter_field(state.cursor_position(), f),
            Modal::Undo => EventResult::pop(state.cursor_position(), action::Pop::One),
            Modal::Delete => EventResult::pop(state.cursor_position(), action::Pop::All),
            _ => EventResult::Handled,
        }
    }

    fn on_exit(&mut self, _state: &mut crate::ui::presenter::State) -> Option<action::Action> {
        // Don't clear the cursor, it'll probably be used by the new state.
        None
    }
}

impl Nav {
    /// Creates a transition to a navigation.
    #[must_use]
    pub fn transition() -> EventResult {
        EventResult::transition(Self {})
    }
}

/// Moves the state cursor according to `c`, if possible.
fn move_cursor(motion: cursor::Motion, state: &mut super::super::State) -> EventResult {
    // TODO(@MattWindsor91): cursor multiplier
    state.move_cursor_by(motion, 1);
    EventResult::Handled
}

/// Constructs an editor entering the given index and field.
fn enter_field(index: usize, field: time::Position) -> EventResult {
    let editor = Editor::new(index, Some(field));
    EventResult::transition(editor)
}
