//! Contains all of the state held by the user interface.

pub mod cursor;
pub mod editor;
pub mod event;
pub mod mode;
pub mod nav;

use crate::model::run;
pub use editor::Editor;

use self::cursor::SplitPosition;

/// The part of zombiesplit that displays and manipulates a model, exposing it
/// to the view.
pub struct Presenter {
    /// The current mode.
    pub mode: Box<dyn mode::Mode>,
    /// The current run.
    pub run: run::Run,
}

impl Presenter {
    /// Constructs a new initial state for a given run.
    #[must_use]
    pub fn new(run: run::Run) -> Self {
        Self {
            mode: Box::new(mode::Inactive),
            run,
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

    /// Handles an event.  Returns true if the event changed the state.
    pub fn handle_event(&mut self, e: &event::Event) {
        match self.mode.handle_event(e, &mut self.run) {
            mode::EventResult::Transition(new_mode) => self.transition(new_mode),
            mode::EventResult::NotHandled => self.handle_event_globally(e),
            mode::EventResult::Handled => (),
        }
    }

    fn handle_event_globally(&mut self, e: &event::Event) {
        use event::Event;
        match e {
            Event::NewRun => self.start_new_run(),
            Event::Quit => self.quit(),
            _ => (),
        }
    }

    fn transition(&mut self, new_mode: Box<dyn mode::Mode>) {
        self.mode.commit(&mut self.run);
        self.mode = new_mode
    }

    /// Starts a new run, abandoning any previous run.
    fn start_new_run(&mut self) {
        // TODO(@MattWindsor91): actually reset the run here.
        self.run.reset();
        let cur = cursor::Cursor::new(self.run.splits.len() - 1);
        // Don't commit the previous mode.
        self.mode = Box::new(nav::Nav::new(cur))
    }

    /// Start the process of quitting.
    fn quit(&mut self) {
        self.transition(Box::new(mode::Quitting))
    }
}
