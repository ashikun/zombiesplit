/*! The presenter part of zombiesplit's model-view-presenter architecture.

The part of zombiesplit that mediates between the model [Session] and the
user interface.

The presenter translates events (ultimately from the keyboard and windowing
system etc.) to operations on the [Session], while translating observations
of changes made to the [Session] into visual and modal changes to the UI.
*/

pub mod cursor;
pub mod event;
pub mod mode;
pub mod state;

use crate::model::{
    attempt::{self, action::Handler, observer, Action},
    short, Time,
};
pub use cursor::Cursor;
pub use mode::Editor;
pub use state::State;
use std::sync::mpsc;

/// A zombiesplit UI presenter, containing all state and modality.
pub struct Presenter<'h, H> {
    /// The current mode.
    pub mode: Box<dyn mode::Mode>,
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
    pub fn new(action_handler: &'h mut H) -> Self {
        Self {
            mode: Box::new(mode::Inactive),
            state: state::State::default(),
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
        if let Some(a) = self.handle_local_event(e) {
            self.action_handler.handle(a);
        }
    }

    fn handle_local_event(&mut self, e: &event::Event) -> Option<Action> {
        match e {
            event::Event::Action(a) => Some(*a),
            event::Event::Modal(m) => self.handle_modal_event(*m),
            event::Event::Quit => {
                self.quit();
                None
            }
        }
    }

    fn handle_modal_event(&mut self, event: event::Modal) -> Option<Action> {
        let ctx = mode::EventContext {
            event,
            state: &mut self.state,
        };
        match self.mode.on_event(ctx) {
            mode::EventResult::Transition(new_mode) => self.transition_with_exit(new_mode),
            mode::EventResult::Action(a) => Some(a),
            mode::EventResult::Handled => None,
        }
    }

    /// Starts the process of quitting.
    fn quit(&mut self) {
        self.transition_with_exit(Box::new(mode::Quitting));
    }

    fn reset(&mut self) {
        let cur = cursor::Cursor::new(self.state.splits.len() - 1);
        // Don't call exit the previous mode's exit hook; it may modify the run
        // in ways we don't want to happen.
        self.transition(Box::new(mode::Nav::new(cur)));
        self.state.reset();
    }

    /// Observes `evt` on this presenter core.
    pub fn observe(&mut self, evt: observer::Event) {
        // TODO(@MattWindsor91): eventually make it possible for this to be
        // called directly as an observe?  the mutability makes it a bit
        // difficult though.

        // TODO(@MattWindsor91): push most of this logic down into the state
        use attempt::observer::Event;
        match evt {
            Event::Total(time, source) => self.state.set_total(time, source),
            Event::NumSplits(count) => self.state.splits.set_split_count(count),
            Event::Reset(_) => self.reset(),
            Event::Attempt(a) => self.state.attempt = a,
            Event::GameCategory(gc) => self.state.game_category = gc,
            Event::Split(short, ev) => {
                self.observe_split(short, ev);
            }
        }
    }

    /// Handles the split event `ev` relating to the split `short`.
    fn observe_split(&mut self, short: short::Name, ev: observer::split::Event) {
        if let observer::split::Event::Time(t, observer::time::Event::Popped) = ev {
            self.open_editor(short, t);
        }
        self.state.handle_split_event(short, ev);
    }

    /// Opens a new split editor at the short named `short`, and preloads it
    /// with the time `time`.
    fn open_editor(&mut self, short: short::Name, time: Time) {
        if let Some(cursor) = self.make_cursor_at(short) {
            let mut editor = Box::new(Editor::new(cursor, None));
            editor.time = time;
            self.transition_with_exit(editor);
        }
    }

    fn make_cursor_at(&self, short: short::Name) -> Option<Cursor> {
        // TODO(@MattWindsor91): update cursor size if the number of splits changes mid-flight
        let max = self.state.splits.len() - 1;
        self.state
            .splits
            .index_of(short)
            .and_then(|pos| Cursor::new_at(pos, max))
    }

    /// Performs a full clean transition between two modes.
    ///
    /// This calls both exit and entry hooks.
    fn transition_with_exit(&mut self, new_mode: Box<dyn mode::Mode>) -> Option<Action> {
        let follow_on = self.mode.on_exit(&mut self.state);
        self.transition(new_mode);
        follow_on
    }

    /// Performs a transition between two modes, calling the entry hook only.
    fn transition(&mut self, new_mode: Box<dyn mode::Mode>) {
        self.mode = new_mode;
        self.mode.on_entry(&mut self.state);
    }
}

/// Used to feed events from an `Observer` into a `Presenter`.
pub struct ModelEventPump(mpsc::Receiver<attempt::observer::Event>);

/// Creates an observer as well as a pump that feeds events from the observer into a presenter.
#[must_use]
pub fn observer() -> (Observer, ModelEventPump) {
    let (send, recv) = mpsc::channel();
    (Observer(send), ModelEventPump(recv))
}

impl<H: Handler> event::Pump<H> for ModelEventPump {
    /// Pumps this event forwarder's event queue, pushing each event to `to`.
    fn pump(&mut self, to: &mut Presenter<H>) {
        self.0.try_iter().for_each(|x| to.observe(x));
    }
}

/// An observer that feeds into a [Presenter].
pub struct Observer(mpsc::Sender<attempt::observer::Event>);

impl attempt::Observer for Observer {
    fn observe(&self, evt: attempt::observer::Event) {
        // TODO(@MattWindsor91): handle errors properly?
        if let Err(e) = self.0.send(evt) {
            log::warn!("error sending event to presenter: {e}");
        }
    }
}
