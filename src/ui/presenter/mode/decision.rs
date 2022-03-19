//! The decision mode.

use super::{
    super::{super::super::model::session, State},
    event, Mode,
};
use std::fmt::{Display, Formatter};

/// A mode that expects a yes/no decision from the user before proceeding.
///
/// The decision chooses one of two event outcomes, which must contain a transition to be
/// well-formed.
pub struct Decision {
    /// The question to pose to the user.
    question: String,
    /// The outcome to resolve to when the decision is 'yes'.
    when_yes: Option<event::Outcome>,
    /// The outcome to resolve to when the decision is 'no'.
    when_no: Option<event::Outcome>,
}

impl Decision {
    /// Constructs a new decision mode with the given question and yes/no events.
    #[must_use]
    pub fn new(
        question: &impl ToString,
        when_yes: event::Outcome,
        when_no: event::Outcome,
    ) -> Self {
        Decision {
            question: question.to_string(),
            // The options here let us perform a safe move on the contents of the outcome.
            when_yes: Some(when_yes),
            when_no: Some(when_no),
        }
    }
}

impl Display for Decision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}? (y/n)", self.question)
    }
}

impl Mode for Decision {
    fn on_entry(&mut self, _state: &mut State) {
        // Intentionally blank
    }

    fn on_event(&mut self, ctx: event::Context) -> event::Outcome {
        match ctx.event {
            event::Event::Decision(true) => self.when_yes.take().unwrap_or_default(),
            event::Event::Decision(false) => self.when_no.take().unwrap_or_default(),
            _ => event::Outcome::default(),
        }
    }

    fn on_exit(&mut self, _state: &mut State) -> Vec<session::Action> {
        vec![]
    }
}
