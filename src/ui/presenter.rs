/*! The presenter part of zombiesplit's model-view-presenter architecture.

The part of zombiesplit that mediates between the model [Session] and the
user interface.

The presenter translates events (ultimately from the keyboard and windowing
system etc.) to operations on the [Session], while translating observations
of changes made to the [Session] into visual and modal changes to the UI.
*/

pub mod event;
pub mod mode;
pub mod state;

use std::sync::mpsc;

pub use event::Event;
pub use mode::Editor;
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
        self.mode.is_running()
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
            event::Event::Action(a) => self.forward_action(*a),
            event::Event::Modal(m) => self.handle_modal_event(*m),
            event::Event::Quit { force } => self.quit(*force),
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
            self.transition(new_mode);
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
        let hard_quit: Box<dyn mode::Mode> = Box::new(mode::Quitting);
        // TODO(@MattWindsor91): fix nesting of quits within quits
        let next_state = move |s| {
            if force {
                hard_quit
            } else {
                Box::new(mode::Decision::new(
                    &"Quit?",
                    mode::event::Outcome::boxed_transition(hard_quit),
                    mode::event::Outcome::boxed_transition(s),
                ))
            }
        };

        self.transition_recursively(next_state);
    }

    fn reset(&mut self, new_attempt: &AttemptInfo) {
        // Don't call exit the previous mode's exit hook; it may modify the run
        // in ways we don't want to happen.
        self.state.reset(new_attempt);
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
            session::Event::Reset(new_attempt) => self.reset(new_attempt),
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
            let mut editor = Box::new(Editor::new(index, None));
            editor.time = time;
            self.transition(editor);
        }
    }

    /// Transitions from one mode to the other.
    fn transition(&mut self, new_mode: Box<dyn mode::Mode>) {
        self.transition_recursively(|_| new_mode);
    }

    /// Performs a transition where the new mode depends on the existing one, calling the entry hook only.
    fn transition_recursively(
        &mut self,
        new_mode_fn: impl FnOnce(Box<dyn mode::Mode>) -> Box<dyn mode::Mode>,
    ) {
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

/// Used to feed events from an `Observer` into a `Presenter`.
pub struct ModelEventPump(mpsc::Receiver<session::event::Event>);

/// Creates an observer as well as a pump that feeds events from the observer into a presenter.
#[must_use]
pub fn observer() -> (Observer, ModelEventPump) {
    let (send, recv) = mpsc::channel();
    (Observer(send), ModelEventPump(recv))
}

impl<H: Handler> event::Pump<H> for ModelEventPump {
    /// Pumps this event forwarder's event queue, pushing each event to `to`.
    fn pump(&mut self, to: &mut Presenter<H>) {
        self.0.try_iter().for_each(|x| to.observe(&x));
    }
}

/// An observer that feeds into a [Presenter].
#[derive(Clone)]
pub struct Observer(mpsc::Sender<session::event::Event>);

impl session::Observer for Observer {
    fn observe(&self, evt: session::event::Event) {
        // TODO(@MattWindsor91): handle errors properly?
        if let Err(e) = self.0.send(evt) {
            log::warn!("error sending event to presenter: {e}");
        }
    }
}
