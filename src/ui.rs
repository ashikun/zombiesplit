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
pub use presenter::event::Pump;
pub use view::View;

use crate::model::attempt::action::{Action, Handler};

/// Top-level user interface instance.
pub struct Instance<'h, E, H, R> {
    events: E,
    view: view::View<R>,
    presenter: presenter::Presenter<'h, H>,
    forwarder: presenter::EventForwarder,
    // TODO(@MattWindsor91): decouple the SDL use here
    limiter: sdl::Limiter,
}

impl<'h, E: presenter::event::Pump<H>, H: Handler, R: view::gfx::Renderer> Instance<'h, E, H, R> {
    /// Runs the UI loop.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to perform an action.
    pub fn run(&mut self) -> Result<()> {
        self.presenter.action_handler.handle(Action::Dump);

        // Initial draw
        self.view.redraw(&self.presenter.state)?;

        while self.presenter.is_running() {
            self.cycle()?;
        }

        Ok(())
    }

    fn cycle(&mut self) -> Result<()> {
        self.forwarder.pump(&mut self.presenter);
        self.events.pump(&mut self.presenter);
        self.view.redraw(&self.presenter.state)?;
        self.limiter.delay();

        Ok(())
    }
}

/// Runs the user interface, configured by `cfg`, over `action_handler`.
///
/// # Errors
///
/// Propagates any errors from creating, spawning, or running the view.
pub fn run(cfg: view::Config, action_handler: &mut impl Handler) -> Result<()> {
    let layout = cfg.layout.clone();
    let sdl = sdl::Manager::new(cfg)?;

    let forwarder = presenter::EventForwarder::new();
    action_handler.add_observer(forwarder.observer());

    let presenter = presenter::Presenter::new(action_handler);
    let mut inst = Instance {
        events: sdl.event_pump()?,
        view: View::new(sdl.renderer()?, &layout),
        presenter,
        forwarder,
        limiter: sdl::Limiter::new(60)?,
    };

    inst.run()
}
