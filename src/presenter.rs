//! Contains all of the state held by the user interface.

pub mod cursor;
pub mod editor;
pub mod event;
pub mod mode;
pub mod nav;

use crate::model::{
    attempt::{self, split::Set, Session},
    comparison::pace,
    game::category::AttemptInfo,
};
pub use editor::Editor;
use std::sync::mpsc;

use self::cursor::SplitPosition;

/// The part of zombiesplit that displays and manipulates a model, exposing it
/// to the view.
pub struct Presenter<'a> {
    /// The current mode.
    pub mode: Box<dyn mode::Mode + 'a>,
    /// The current run.
    pub session: Session<'a>,
    pub state: State,
    obs_receiver: mpsc::Receiver<attempt::observer::Event>,
    obs_sender: mpsc::Sender<attempt::observer::Event>,
}

#[derive(Debug, Default)]
pub struct State {
    /// The current attempt information.
    pub attempt: AttemptInfo,
    // TODO(@MattWindsor91): move things from using Session to using this.
}

impl<'a> Presenter<'a> {
    /// Constructs a new initial state for a given session.
    #[must_use]
    pub fn new(session: Session<'a>) -> Self {
        // TODO(@MattWindsor91): remove session use here
        let (obs_sender, obs_receiver) = mpsc::channel();
        let mut p = Self {
            mode: Box::new(mode::Inactive),
            state: State::default(),
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

    /// Gets the run pace up to and excluding the cursor, if any.
    #[must_use]
    pub fn run_pace(&self) -> pace::Pair {
        self.mode.cursor().map_or(pace::Pair::default(), |c| {
            self.session.paced_time_at(c.position())
        })
    }

    /// Handles an event.
    ///
    /// Events are offered to the current mode first, and handled globally if
    /// the event is refused by the mode.
    pub fn handle_event(&mut self, e: &event::Event) {
        match self.mode.handle_event(e, &mut self.session) {
            mode::EventResult::Transition(new_mode) => self.transition(new_mode),
            mode::EventResult::NotHandled => self.handle_event_globally(e),
            mode::EventResult::Handled => (),
        }
    }

    fn handle_event_globally(&mut self, e: &event::Event) {
        use event::Event;
        match e {
            Event::Commit => self.mode.commit(&mut self.session),
            Event::NewRun => self.start_new_run(),
            Event::Quit => self.quit(),
            _ => (),
        }
    }

    fn transition(&mut self, new_mode: Box<dyn mode::Mode>) {
        self.mode.commit(&mut self.session);
        self.mode = new_mode
    }

    /// Starts a new run, abandoning any previous run.
    fn start_new_run(&mut self) {
        self.session.reset();
    }

    /// Start the process of quitting.
    fn quit(&mut self) {
        self.transition(Box::new(mode::Quitting))
    }

    pub fn pump(&mut self) {
        let events = self.obs_receiver.try_iter();
        for l in events {
            use attempt::observer::Event;
            match l {
                Event::Reset(_) => {
                    let cur = cursor::Cursor::new(self.session.num_splits() - 1);
                    // Don't commit the previous mode.
                    self.mode = Box::new(nav::Nav::new(cur))
                }
                Event::NewAttempt(a) => self.state.attempt = a,
            }
        }
    }
}

pub struct Observer {
    sender: mpsc::Sender<attempt::observer::Event>,
}

impl attempt::Observer for Observer {
    fn observe(&self, evt: attempt::observer::Event) {
        // TODO(@MattWindsor91): handle errors properly?
        if let Err(e) = self.sender.send(evt) {
            log::warn!("error sending event to presenter: {}", e)
        }
    }
}
