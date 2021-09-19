//! Contains all of the state held by the user interface.

pub mod cursor;
pub mod editor;
pub mod event;
pub mod mode;
pub mod nav;
pub mod state;

use crate::model::attempt::{self, Session};
pub use editor::Editor;
use std::sync::mpsc;

use self::cursor::SplitPosition;

/// The part of zombiesplit that mediates between the model [Session] and the
/// user interface.
///
/// The presenter translates events (ultimately from the keyboard and windowing
/// system etc.) to operations on the [Session], while translating observations
/// of changes made to the [Session] into visual and modal changes to the UI.
pub struct Presenter<'a> {
    /// The current mode.
    pub mode: Box<dyn mode::Mode + 'a>,
    /// The zombiesplit session being controlled by the presenter.
    pub session: Session<'a>,
    pub state: state::State,
    obs_receiver: mpsc::Receiver<attempt::observer::Event>,
    obs_sender: mpsc::Sender<attempt::observer::Event>,
}
impl<'a> Presenter<'a> {
    /// Constructs a new initial state for a given session.
    #[must_use]
    pub fn new(session: Session<'a>) -> Self {
        // TODO(@MattWindsor91): remove session use here
        let (obs_sender, obs_receiver) = mpsc::channel();
        let mut p = Self {
            mode: Box::new(mode::Inactive),
            state: state::State::default(),
            session,
            obs_sender,
            obs_receiver,
        };
        p.session.observers.add(Box::new(p.observer()));
        p.session.dump_to_observers();
        p
    }

    /// Gets an observer that can be used to update the presenter with
    /// session changes.
    #[must_use]
    pub fn observer(&self) -> Observer {
        Observer {
            sender: self.obs_sender.clone(),
        }
    }

    /// Gets the split position, if any.
    #[must_use]
    pub fn split_position(&self, index: usize) -> SplitPosition {
        self.mode
            .cursor()
            .map_or(SplitPosition::default(), |x| x.split_position(index))
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
        match self.mode.handle_event(e, &mut self.session) {
            mode::EventResult::Transition(new_mode) => self.transition(new_mode),
            mode::EventResult::NotHandled => self.handle_event_globally(e),
            mode::EventResult::Handled => self.refresh_state_cursor(),
        }
    }

    fn handle_event_globally(&mut self, e: &event::Event) {
        use event::Event;
        match e {
            Event::Commit => self.commit_mode(),
            Event::NewRun => self.session.reset(),
            Event::Quit => self.quit(),
            _ => (),
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
            .set_cursor(self.mode.cursor().map(|x| x.position()))
    }

    /// Start the process of quitting.
    fn quit(&mut self) {
        self.transition(Box::new(mode::Quitting));
    }

    pub fn pump(&mut self) {
        let events = self.obs_receiver.try_iter();
        for l in events {
            use attempt::observer::Event;
            match l {
                Event::AddSplit(short, name) => self.state.add_split(short, name),
                Event::Reset(_) => {
                    let cur = cursor::Cursor::new(self.state.num_splits() - 1);
                    // Don't commit the previous mode.
                    self.mode = Box::new(nav::Nav::new(cur));
                    self.state.reset();
                }
                Event::Attempt(a) => self.state.attempt = a,
                Event::GameCategory(gc) => self.state.game_category = gc,
                Event::Split(short, ev) => self.state.handle_split_event(short, ev),
            }
        }
        // TODO(@MattWindsor91): this is wasteful but borrowck won't let me do better yet.
        self.refresh_state_cursor();
    }
}

pub struct Observer {
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
