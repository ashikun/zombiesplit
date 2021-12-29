/*! The top-level UI module.

This UI presents, and allows in-flight modifications to, run attempts.  It can
be attached to the database to allow finished attempts to be committed.

The UI itself has a roughly model-view-presenter layout (with the downstream
attempt session forming the model).
*/

pub mod error;
pub mod presenter;
pub mod sdl;
pub mod view;

pub use error::{Error, Result};
pub use presenter::event::Pump;
pub use view::View;

use crate::model::attempt::action::{Action, Handler};

/// Trait for things that provide UI components such as event pumps and renderers.
pub trait Manager<'r> {
    type Pump;
    type Renderer: 'r;

    /// Constructs an event pump.
    ///
    /// # Errors
    ///
    /// Fails if the underlying UI backend can't construct an event pump.
    fn event_pump(&self) -> Result<Self::Pump>;

    /// Constructs a renderer.
    ///
    /// # Errors
    ///
    /// Fails if the underlying UI backend can't construct a renderer.
    fn renderer(&'r self) -> Result<Self::Renderer>;
}

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
    /// Constructs a new UI instance using the configuration in `config`,
    ///
    /// # Errors
    ///
    /// Fails if the manager can't construct certain components of the UI.
    pub fn new(
        cfg: &view::Config,
        manager: &'h impl Manager<'h, Pump = E, Renderer = R>,
        action_handler: &'h mut H,
        forwarder: presenter::EventForwarder,
    ) -> Result<Self> {
        Ok(Self {
            events: manager.event_pump()?,
            view: View::new(manager.renderer()?, &cfg.layout),
            presenter: presenter::Presenter::new(action_handler),
            forwarder,
            limiter: sdl::Limiter::new(60)?,
        })
    }

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
