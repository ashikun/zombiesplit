/*! The presenter part of zombiesplit's model-view-presenter architecture.

The part of zombiesplit that mediates between the model [Session] and the
user interface.

The presenter translates events (ultimately from the keyboard and windowing
system etc.) to operations on the [Session], while translating observations
of changes made to the [Session] into visual and modal changes to the UI.
*/

pub mod event;
pub mod mode;
pub mod observer;
pub mod state;

pub use event::Event;
pub use state::State;

use super::super::model::{
    game::category::AttemptInfo,
    session::{self, action::Handler, Action},
    short, Time,
};

/// A zombiesplit UI presenter, containing all state and modality.
pub struct Presenter<'h, H> {
    /// The current mode.
    mode: Box<dyn mode::Mode>,
    /// The handler used to perform actions on the splitter session.
    pub action_handler: &'h mut H,
    /// The visual state being updated by the presenter.
    pub state: state::State,
}

impl<'h, H: Handler> Presenter<'h, H> {
    /// Constructs a new presenter over a given action handler.
    ///
    /// The presenter can be used as an observer by feeding it events through, for instance, an
    /// `EventForwarder`.
    #[must_use]
    pub fn new(action_handler: &'h mut H, dump: &session::State) -> Self {
        Self {
            mode: Box::new(mode::Nav),
            state: State::from_dump(dump),
            action_handler,
        }
    }

    /// Gets whether the UI should be running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        !matches!(self.mode.mode_type(), mode::Type::Quitting)
    }

    /// Handles an event `e`.
    ///
    /// Action events are forwarded directly to the session.
    ///
    /// Modal events are handled by the current mode, which may interpret them
    /// as an internal change, request an action on the session, or request a
    /// transition to another mode.
    ///
    /// Any other events are handled directly.
    pub fn handle_event(&mut self, e: &event::Event) {
        match e {
            event::Event::Modal(m) => self.handle_modal_event(*m),
            event::Event::Quit { force } => self.quit(*force),
            event::Event::Reset => self.reset(),
        };
        // This is slightly inefficient, as we don't always change the mode when an event happens.
        self.update_mode_line();
    }

    /// Updates the state's representation of which mode the presenter is in.
    fn update_mode_line(&mut self) {
        self.state.mode = self.mode.to_string();
    }

    fn handle_modal_event(&mut self, event: mode::Event) {
        let ctx = mode::event::Context {
            event,
            state: &mut self.state,
        };
        let mode::event::Outcome { actions, next_mode } = self.mode.on_event(ctx);
        self.forward_actions(actions);
        if let Some(new_mode) = next_mode {
            self.transition(|_| new_mode);
        }
    }

    fn forward_actions(&mut self, xs: impl IntoIterator<Item = Action>) {
        for x in xs {
            self.forward_action(x);
        }
    }

    fn forward_action(&mut self, x: Action) {
        // TODO(@MattWindsor91): handle this properly
        if let Err(e) = self.action_handler.handle(x) {
            log::error!("error handling action: {}", e.to_string());
        }
    }

    /// Starts the process of quitting (or asking the user if they want to quit).
    fn quit(&mut self, force: bool) {
        if let mode::Type::Normal = self.mode.mode_type() {
            let hard_quit: Box<dyn mode::Mode> = Box::new(mode::Quitting);
            let next_state = move |s| {
                if force {
                    hard_quit
                } else {
                    soft_quit(hard_quit, s)
                }
            };

            self.transition(next_state);
        }
    }

    /// Starts the process of resetting.
    fn reset(&mut self) {
        use session::action::OldDestination;
        self.transition(|_| {
            // TODO(@MattWindsor91): only ask if we want to discard if we have splits
            Box::new(mode::Decision::new(
                &"Save this run?",
                reset_outcome(OldDestination::Save),
                reset_outcome(OldDestination::Discard),
            ))
        });
    }

    /// Observes `evt` on this presenter core.
    pub fn observe(&mut self, ev: &session::Event) {
        // TODO(@MattWindsor91): eventually make it possible for this to be
        // called directly as an observe?  the mutability makes it a bit
        // difficult though.

        self.observe_locally(ev);
        self.state.handle_event(ev);
    }

    /// Observes `evt` on the presenter.
    fn observe_locally(&mut self, ev: &session::Event) {
        match ev {
            session::Event::Split(short, ev) => self.observe_split(*short, ev),
            session::Event::Reset(new_attempt) => self.observe_reset(new_attempt),
            _ => (),
        };
    }

    /// Handles the split event `ev` relating to the split `short`.
    fn observe_split(&mut self, short: short::Name, ev: &session::event::Split) {
        // This implements a true 'pop' type semantics whereby pops of one time open an editor.
        if let session::event::Split::Popped(session::action::Pop::One) = ev {
            if let Some(t) = self.last_time_at(short) {
                self.open_editor(short, t);
            }
        }
    }

    fn last_time_at(&mut self, sid: short::Name) -> Option<Time> {
        self.state
            .split_at_short(sid)
            .and_then(|s| s.times.last())
            .copied()
    }

    /// Opens a new split editor at the short named `short`, and preloads it
    /// with the time `time`.
    fn open_editor(&mut self, short: short::Name, time: Time) {
        if let Some(index) = self.state.index_of_split(short) {
            let mut editor = Box::new(mode::Editor::new(index, None));
            editor.time = time;
            let new_mode = editor;
            self.transition(|_| new_mode);
        }
    }

    fn observe_reset(&mut self, new_attempt: &AttemptInfo) {
        // Don't call exit the previous mode's exit hook; it may modify the run
        // in ways we don't want to happen.
        self.state.reset(new_attempt);
    }

    /// Transitions from one mode to another, determined in terms of the old mode.
    fn transition(&mut self, new_mode_fn: impl FnOnce(Box<dyn mode::Mode>) -> Box<dyn mode::Mode>) {
        // We can't use 'transition' here because we need to call on_exit before doing this swap
        let actions = self.mode.on_exit(&mut self.state);

        // TODO(@MattWindsor91): surely there must be a better way of doing this
        let mut tmp: Box<dyn mode::Mode> = Box::new(mode::Quitting);
        std::mem::swap(&mut tmp, &mut self.mode);
        self.mode = (new_mode_fn)(tmp);
        self.mode.on_entry(&mut self.state);
        self.forward_actions(actions);
    }
}

fn reset_outcome(dest: session::action::OldDestination) -> mode::event::Outcome {
    mode::event::Outcome {
        actions: vec![session::Action::NewRun(dest)],
        next_mode: Some(Box::new(mode::Nav)),
    }
}

fn soft_quit(quitter: Box<dyn mode::Mode>, old_state: Box<dyn mode::Mode>) -> Box<dyn mode::Mode> {
    Box::new(mode::Decision::transition(&"Quit?", quitter, old_state))
}
