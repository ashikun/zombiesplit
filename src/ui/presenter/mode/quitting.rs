//! The [Quitting] mode.

use super::{super::super::super::model::session, event, Mode, State};
use std::fmt::{Display, Formatter};

/// Mode for when we are quitting.
pub struct Quitting;

impl Display for Quitting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("quit")
    }
}

impl Mode for Quitting {
    fn on_entry(&mut self, _state: &mut State) {}

    fn on_event(&mut self, _ctx: event::Context) -> event::Outcome {
        event::Outcome::default()
    }

    fn on_exit(&mut self, _state: &mut State) -> Vec<session::Action> {
        unreachable!("should not be able to exit out of the Quitting state")
    }

    fn mode_type(&self) -> super::Type {
        super::Type::Quitting
    }
}
