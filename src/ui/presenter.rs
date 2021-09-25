/*! The presenter part of zombiesplit's model-view-presenter architecture.

The part of zombiesplit that mediates between the model [Session] and the
user interface.

The presenter translates events (ultimately from the keyboard and windowing
system etc.) to operations on the [Session], while translating observations
of changes made to the [Session] into visual and modal changes to the UI.

For borrowck reasons, the presenter is split into two structs: [Presenter]
and [Core].
*/

pub mod cursor;
pub mod editor;
pub mod event;
pub mod mode;
pub mod nav;
pub mod state;

use crate::model::{
    attempt::{self, observer, Session},
    short,
};
pub use editor::Editor;
pub use state::State;
use std::{rc::Rc, sync::mpsc};

/// The core of a zombiesplit presenter, containing all state and modality.

pub struct Core<'a> {
    /// The current mode.
    pub mode: Box<dyn mode::Mode + 'a>,
    /// The zombiesplit session being controlled by the presenter.
    pub session: Session<'a>,
    pub state: state::State,
}

impl<'a> Core<'a> {
    /// Constructs a new initial state for a given session.
    #[must_use]
    pub fn new(session: Session<'a>) -> Self {
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

    /// Borrows the current editor (immutably), if one exists.
    #[must_use]
    pub fn editor(&self) -> Option<&Editor> {
        self.mode.editor()
    }

    /// Handles an event.
    ///
    /// Events are offered to the current mode first, and handled globally if
    /// the event is refused by the mode.
    pub fn handle_event(&mut self, e: &event::Event) {
        if let Some(a) = self.handle_potentially_modal_event(e) {
            self.handle_attempt_event(a);
        }
    }

    fn handle_potentially_modal_event(&mut self, e: &event::Event) -> Option<event::Attempt> {
        match e {
            event::Event::Attempt(a) => Some(*a),
            event::Event::Modal(m) => self.handle_modal_event(*m),
        }
    }

    fn handle_modal_event(&mut self, m: event::Modal) -> Option<event::Attempt> {
        match self.mode.handle_event(&m, &mut self.session) {
            mode::EventResult::Transition(new_mode) => {
                self.transition(new_mode);
                None
            }
            mode::EventResult::Expanded(a) => Some(a),
            mode::EventResult::Handled => {
                self.refresh_state_cursor();
                None
            }
        }
    }

    fn handle_attempt_event(&mut self, e: event::Attempt) {
        use event::Attempt;
        match e {
            Attempt::NewRun => self.session.reset(),
            Attempt::Push(pos, time) => self.session.push_to(pos, time),
            Attempt::Pop(pos) => {
                self.session.pop_from(pos);
            }
            Attempt::Clear(pos) => self.session.clear_at(pos),
            Attempt::Quit => self.quit(),
        }
    }

    fn transition(&mut self, new_mode: Box<dyn mode::Mode>) {
        self.commit_mode();
        self.mode = new_mode;
    }

    fn commit_mode(&mut self) {
        self.mode.commit(&mut self.session);
    }

    /// Refreshes the presenter state's view of the cursor.
    ///
    /// This is done after any presenter event that may have moved the cursor,
    /// since various parts of the state (eg totals) depend on the cursor
    /// position being available outside of the mode.
    fn refresh_state_cursor(&mut self) {
        // TODO(@MattWindsor91): get rid of this, somehow.
        self.state
            .set_cursor(self.mode.cursor().map(cursor::Cursor::position));

        // TODO(@MattWindsor91): this too is a hack.
        if let Some(c) = self.state.cursor_pos {
            for s in &mut self.state.splits {
                s.set_editor(None);
            }
            self.state.splits[c].set_editor(self.mode.editor());
        }
    }

    /// Start the process of quitting.
    fn quit(&mut self) {
        self.transition(Box::new(mode::Quitting));
    }

    /// Handles the split event `ev` relating to
    fn handle_split_event(&mut self, short: short::Name, ev: observer::split::Event) {
        self.state.handle_split_event(short, ev);

        if let observer::split::Event::Time(_, observer::time::Event::Popped) = ev {
            // We just popped a time, so we should open it into an editor.
            if let Some(cursor) = self.mode.cursor().copied() {
                self.transition(Box::new(Editor::new(cursor, None)));
            }
        }
    }

    fn reset(&mut self) {
        let cur = cursor::Cursor::new(self.state.num_splits() - 1);
        // Don't commit the previous mode.
        self.mode = Box::new(nav::Nav::new(cur));
        self.state.reset();
    }

    /// Observes `evt` on this presenter core.
    pub fn observe(&mut self, evt: observer::Event) {
        // TODO(@MattWindsor91): eventually make it possible for this to be
        // called directly as an observe?  the mutability makes it a bit
        // difficult though.
        use attempt::observer::Event;
        match evt {
            Event::AddSplit(short, name) => self.state.add_split(short, name),
            Event::Reset(_) => self.reset(),
            Event::Attempt(a) => self.state.attempt = a,
            Event::GameCategory(gc) => self.state.game_category = gc,
            Event::Split(short, ev) => {
                self.handle_split_event(short, ev);
            }
        }
        // TODO(@MattWindsor91): this is wasteful but borrowck won't let me do better yet.
        self.refresh_state_cursor();
    }
}

/// A functional presenter, with the ability to listen to observations from the
/// underlying session.
pub struct Presenter<'a> {
    /// The underlying core.
    pub core: Core<'a>,
    obs_receiver: mpsc::Receiver<attempt::observer::Event>,
    /// Keeps the observer feeding `obs_receiver` alive.
    obs_sender: Rc<dyn attempt::Observer>,
}

impl<'a> Presenter<'a> {
    /// Lifts a presenter into an observable presenter.
    ///
    /// This installs an observer into the [Session] that allows
    /// events to be fed asynchronously into the [Core].
    #[must_use]
    pub fn new(core: Core<'a>) -> Self {
        let (obs_sender, obs_receiver) = mpsc::channel();
        let obs_sender: Rc<dyn attempt::Observer> = Rc::new(Observer { sender: obs_sender });
        let mut o = Self {
            core,
            obs_receiver,
            obs_sender,
        };
        o.core.session.observers.add(Rc::downgrade(&o.obs_sender));
        o.core.session.dump_to_observers();
        o
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
