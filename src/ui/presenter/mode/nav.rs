//! The [Nav] struct and its implementations.

use crate::model::{attempt::Action, time};

use super::{
    super::state::{cursor, State},
    editor::Editor,
    event::Modal,
    EventContext, EventResult, Mode,
};

/// Mode for when we are navigating splits.
pub struct Nav;

impl Mode for Nav {
    fn on_entry(&mut self, _state: &mut State) {}

    fn on_event(&mut self, EventContext { event, state, .. }: EventContext) -> EventResult {
        match event {
            Modal::Cursor(c) => move_cursor(c, state),
            Modal::EnterField(f) => enter_field(state.cursor_position(), f),
            Modal::Undo => EventResult::Action(Action::Pop(state.cursor_position())),
            Modal::Delete => EventResult::Action(Action::Clear(state.cursor_position())),
            _ => EventResult::Handled,
        }
    }

    fn on_exit(&mut self, _state: &mut crate::ui::presenter::State) -> Option<Action> {
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
