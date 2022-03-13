//! The [Nav] struct and its implementations.

use std::fmt::{Display, Formatter};

use super::{
    super::{
        super::super::model::{session::action, timing::time},
        state::{cursor, State},
    },
    editor::Editor,
    event, Mode,
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

    fn on_event(&mut self, event::Context { event, state, .. }: event::Context) -> event::Outcome {
        match event {
            event::Event::Cursor(c) => move_cursor(c, state),
            event::Event::EnterField(f) => enter_field(state.cursor_position(), f),
            event::Event::Undo => event::Outcome::pop(state.cursor_position(), action::Pop::One),
            event::Event::Delete => event::Outcome::pop(state.cursor_position(), action::Pop::All),
            _ => event::Outcome::Handled,
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
    pub fn transition() -> event::Outcome {
        event::Outcome::transition(Self {})
    }
}

/// Moves the state cursor according to `c`, if possible.
fn move_cursor(motion: cursor::Motion, state: &mut super::super::State) -> event::Outcome {
    // TODO(@MattWindsor91): cursor multiplier
    state.move_cursor_by(motion, 1);
    event::Outcome::Handled
}

/// Constructs an editor entering the given index and field.
fn enter_field(index: usize, field: time::Position) -> event::Outcome {
    let editor = Editor::new(index, Some(field));
    event::Outcome::transition(editor)
}
