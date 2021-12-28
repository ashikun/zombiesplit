/*! The top-level UI module.

This UI presents, and allows in-flight modifications to, run attempts.  It can
be attached to the database to allow finished attempts to be committed.

The UI itself has a roughly model-view-presenter layout (with the downstream
attempt session forming the model).
*/

pub mod error;
pub mod presenter;
mod sdl;
pub mod view;

pub use error::{Error, Result};
pub use presenter::EventForwarder;
pub use view::View;

/// Top-level user interface instance.
pub struct Instance<'s, 'p, E, R> {
    events: E,
    view: view::View<R>,
    presenter: presenter::EventForwarder<'s, 'p>,
}

impl<'s, 'p, E: presenter::event::Pump, R: view::gfx::Renderer> Instance<'s, 'p, E, R> {
    /// Runs the UI loop.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to perform an action.
    pub fn run(&mut self) -> Result<()> {
        // Initial draw
        self.view.redraw(&self.presenter.core.state)?;

        while self.presenter.core.is_running() {
            self.cycle()?;
        }

        Ok(())
    }

    fn cycle(&mut self) -> Result<()> {
        self.presenter.pump();
        self.events.pump(&mut self.presenter.core);
        self.view.redraw(&self.presenter.core.state)?;

        std::thread::sleep(std::time::Duration::from_millis(1));

        Ok(())
    }
}

/// Runs the user interface, configured by `cfg`, over `session`.
///
/// # Errors
///
/// Propagates any errors from creating, spawning, or running the view.
pub fn run(cfg: view::Config, session: &mut crate::model::attempt::Session) -> Result<()> {
    let layout = cfg.layout.clone();
    let sdl = sdl::Manager::new(cfg)?;
    let presenter = presenter::Presenter::new(session);
    let mut inst = Instance {
        events: sdl.event_pump()?,
        view: View::new(sdl.renderer()?, &layout),
        presenter: EventForwarder::new(presenter),
    };
    inst.run()
}
