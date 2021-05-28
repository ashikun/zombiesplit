//! Contains all of the state held by the user interface.

pub mod cursor;
pub mod editor;
pub mod event;
pub mod mode;
pub mod split;

use crate::model::run;
pub use editor::Editor;

/// The part of zombiesplit that displays and manipulates a model, exposing it
/// to the view.
pub struct Presenter {
    /// The current mode.
    pub mode: Box<dyn mode::Mode>,
    /// The current run.
    pub run: run::Run,
    /// Whether the state is dirty.
    pub is_dirty: bool,
}

impl Presenter {
    /// Constructs a new initial state for a given run.
    #[must_use]
    pub fn new(run: run::Run) -> Self {
        Self {
            mode: Box::new(mode::Inactive),
            run,
            is_dirty: false,
        }
    }

    /// Produces a vector of split references.
    #[must_use]
    pub fn splits(&self) -> Vec<split::Ref> {
        self.run
            .splits
            .iter()
            .enumerate()
            .map(|(index, split)| split::Ref {
                index,
                split,
                presenter: self,
            })
            .collect()
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
        use event::Event;
        match e {
            Event::NewRun => self.start_new_run(),
            Event::Quit => self.quit(),
            _ => self.delegate_event(e),
        }
    }

    /// Delegates an event to the mode.
    fn delegate_event(&mut self, e: &event::Event) {
        match self.mode.handle_event(e) {
            mode::EventResult::Transition(new_mode) => self.transition(new_mode),
            mode::EventResult::Handled => self.dirty(),
            mode::EventResult::NotHandled => (),
        }
    }

    fn transition(&mut self, new_mode: Box<dyn mode::Mode>) {
        self.mode.commit(&mut self.run);
        self.mode = new_mode;
        self.dirty()
    }

    /// Starts a new run, abandoning any previous run.
    fn start_new_run(&mut self) {
        // TODO(@MattWindsor91): actually reset the run here.
        let cur = cursor::Cursor::new(self.run.splits.len() - 1);
        self.transition(Box::new(mode::Nav::new(cur)))
    }

    /// Start the process of quitting.
    fn quit(&mut self) {
        self.transition(Box::new(mode::Quitting))
    }

    // Marks the UI as dirty.
    fn dirty(&mut self) {
        self.is_dirty = true
    }
}
