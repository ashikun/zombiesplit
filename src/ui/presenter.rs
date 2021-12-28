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
    attempt::{self, observer, Action, Session},
    short, Time,
};
pub use cursor::Cursor;
pub use mode::Editor;
pub use state::State;
use std::{rc::Rc, sync::mpsc};

/// A zombiesplit UI presenter, containing all state and modality.
pub struct Presenter<'s, 'cmp> {
    /// The current mode.
    pub mode: Box<dyn mode::Mode>,
    /// The zombiesplit session being controlled by the presenter.
    pub session: &'s mut Session<'cmp>,
    /// The visual state being updated by the presenter.
    pub state: state::State,
}

impl<'s, 'cmp> Presenter<'s, 'cmp> {
    /// Constructs a new initial state for a given session.
    #[must_use]
    pub fn new(session: &'s mut Session<'cmp>) -> Self {
        // TODO(@MattWindsor91): remove session use here
        Self {
            mode: Box::new(mode::Inactive),
            state: state::State::default(),
            session,
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
            self.session.perform(a);
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
        let cur = cursor::Cursor::new(self.state.num_splits() - 1);
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
        use attempt::observer::Event;
        match evt {
            Event::Total(time, source) => self.state.set_total(time, source),
            Event::AddSplit(short, name) => self.state.add_split(short, name),
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
        self.state.handle_split_event(short, ev);

        if let observer::split::Event::Time(t, observer::time::Event::Popped) = ev {
            self.open_editor(short, t);
        }
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
        let max = self.session.num_splits() - 1;
        self.session
            .position_of(short)
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

/// A functional presenter, with the ability to listen to observations from the
/// underlying session.
pub struct EventForwarder<'s, 'cmp> {
    /// The underlying core.
    pub core: Presenter<'s, 'cmp>,
    obs_receiver: mpsc::Receiver<attempt::observer::Event>,
    /// Keeps the observer feeding `obs_receiver` alive.
    obs_sender: Rc<dyn attempt::Observer>,
}

impl<'s, 'cmp> EventForwarder<'s, 'cmp> {
    /// Lifts a presenter into an observable presenter.
    ///
    /// This installs an observer into the [Session] that allows
    /// events to be fed asynchronously into the [Core].
    #[must_use]
    pub fn new(core: Presenter<'s, 'cmp>) -> Self {
        let (obs_sender, obs_receiver) = mpsc::channel();
        let obs_sender: Rc<dyn attempt::Observer> = Rc::new(Observer { sender: obs_sender });
        let o = Self {
            core,
            obs_receiver,
            obs_sender,
        };
        // TODO(@MattWindsor91): split this bit out, make it part of the UI bootup.
        o.core.session.observers.add(o.observer());
        o.core.session.dump_to_observers();
        o
    }

    /// Gets this presenter as an observer.
    #[must_use]
    pub fn observer(&self) -> std::rc::Weak<dyn attempt::Observer> {
        // TODO(@MattWindsor91): this will be exposed as public once the presenter no longer takes
        // the entire session in as a mutable borrow.
        Rc::downgrade(&self.obs_sender)
    }

    pub fn pump(&mut self) {
        let events = self.obs_receiver.try_iter();
        for l in events {
            self.core.observe(l);
        }
    }
}

/// An observer that feeds into a [Presenter].
struct Observer {
    sender: mpsc::Sender<attempt::observer::Event>,
}

impl attempt::Observer for Observer {
    fn observe(&self, evt: attempt::observer::Event) {
        // TODO(@MattWindsor91): handle errors properly?
        if let Err(e) = self.sender.send(evt) {
            log::warn!("error sending event to presenter: {}", e);
        }
    }
}
