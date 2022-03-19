/*! The top-level UI module.

This UI presents, and allows in-flight modifications to, run attempts.  It can
be attached to the database to allow finished attempts to be committed.

The UI itself has a roughly model-view-presenter layout (with the downstream
attempt session forming the model).
*/

pub mod error;
mod event;
pub mod presenter;
pub mod sdl;
pub mod view;

pub use error::{Error, Result};
pub use view::View;

use crate::model::session::action::Handler;

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
    view: view::View<'h, R>,
    presenter: presenter::Presenter<'h, H>,
    forwarder: presenter::observer::Pump,
    // TODO(@MattWindsor91): decouple the SDL use here
    limiter: sdl::Limiter,
}

impl<'m, E: event::Pump, H: Handler, R: view::gfx::Renderer> Instance<'m, E, H, R> {
    /// Constructs a new UI instance using the configuration in `config`,
    ///
    /// # Errors
    ///
    /// Fails if the manager can't construct certain components of the UI.
    pub fn new(
        cfg: &'m view::Config,
        manager: &'m impl Manager<'m, Pump = E, Renderer = R>,
        presenter: presenter::Presenter<'m, H>,
        forwarder: presenter::observer::Pump,
    ) -> Result<Self> {
        Ok(Self {
            events: manager.event_pump()?,
            view: View::new(manager.renderer()?, &cfg.layout),
            presenter,
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
        // Initial draw
        self.view.redraw(&self.presenter.state)?;

        while self.presenter.is_running() {
            self.cycle()?;
        }

        Ok(())
    }

    fn cycle(&mut self) -> Result<()> {
        self.forwarder.pump(&mut self.presenter);

        self.pump_events();

        self.view.redraw(&self.presenter.state)?;

        // TODO(@MattWindsor91): this effectively limits how quickly we can respond to network
        // traffic to 60fps, too!
        self.limiter.delay();

        Ok(())
    }

    fn pump_events(&mut self) {
        for e in self.events.pump() {
            match e {
                event::Event::View(e) => self.view.handle_event(&e),
                event::Event::Presenter(e) => self.presenter.handle_event(&e),
            };
        }
    }
}
