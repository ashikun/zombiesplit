//! Contains all of the state held by the user interface.

pub mod cursor;
pub mod editor;
pub mod event;
pub mod mode;
pub mod nav;

use crate::model::{
    attempt::{split::Set, Session},
    comparison::pace,
};
pub use editor::Editor;

use self::cursor::SplitPosition;

/// The part of zombiesplit that displays and manipulates a model, exposing it
/// to the view.
pub struct Presenter<'a> {
    /// The current mode.
    pub mode: Box<dyn mode::Mode + 'a>,
    /// The current run.
    pub session: Session<'a>,
}

impl<'a> Presenter<'a> {
    /// Constructs a new initial state for a given session.
    #[must_use]
    pub fn new(session: Session<'a>) -> Self {
        Self {
            mode: Box::new(mode::Inactive),
            session,
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
        let cur = cursor::Cursor::new(self.session.num_splits() - 1);
        // Don't commit the previous mode.
        self.mode = Box::new(nav::Nav::new(cur))
    }

    /// Start the process of quitting.
    fn quit(&mut self) {
        self.transition(Box::new(mode::Quitting))
    }
}
