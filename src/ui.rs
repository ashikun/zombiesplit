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

use std::cell::RefCell;

pub use error::{Error, Result};
pub use presenter::Presenter;
pub use view::View;

/// Manages top-level UI resources.
pub struct Manager<'c> {
    sdl: sdl2::Sdl,
    screen: RefCell<sdl2::render::Canvas<sdl2::video::Window>>,
    textures: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    /// The system view configuration, which is borrowing parts of a config file.
    cfg: view::Config<'c>,
}

impl<'c> Manager<'c> {
    /// Creates a new view, opening a window in the process.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the SDL subsystems the UI manager requires
    /// fail to initialise.
    pub fn new(cfg: view::Config<'c>) -> Result<Self> {
        let sdl = sdl2::init().map_err(Error::Init)?;
        let video = sdl.video().map_err(Error::Init)?;
        let window = view::gfx::make_window(&video, cfg.window)?;
        let screen = window.into_canvas().build().map_err(Error::SdlInteger)?;
        let textures = screen.texture_creator();
        Ok(Self {
            sdl,
            screen: RefCell::new(screen),
            textures,
            cfg,
        })
    }

    /// Spawns a [Core] handling UI services.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL can't spawn an event pump.
    pub fn spawn<'p>(&self, presenter: presenter::Presenter<'p>) -> Result<Instance<'_, 'p>> {
        let font_config = self.cfg.fonts.resolve()?;
        let font_manager =
            sdl::font::Manager::new(&self.textures, font_config, &self.cfg.colours.fg);
        let renderer = sdl::Renderer::new(
            self.screen.borrow_mut(),
            self.cfg.window,
            font_manager,
            &self.cfg.colours,
        );
        let view = view::View::new(renderer, self.cfg.window);

        let events = self.sdl.event_pump().map_err(Error::Init)?;

        Ok(Instance {
            events,
            view,
            presenter,
        })
    }
}

/// An instance of the view for a particular presenter.
pub struct Instance<'c, 'p> {
    events: sdl2::EventPump,
    view: view::View<sdl::Renderer<'c>>,
    presenter: presenter::Presenter<'p>,
}

impl<'c, 'p> Instance<'c, 'p> {
    /// Runs the UI loop.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to perform an action.
    pub fn run(&mut self) -> Result<()> {
        self.view.redraw(&self.presenter.core.state)?;

        while self.presenter.core.is_running() {
            self.cycle()?;
        }

        Ok(())
    }

    fn cycle(&mut self) -> Result<()> {
        self.presenter.pump();
        for e in self.events.poll_iter() {
            if let Some(x) = sdl::event::from_sdl(&e) {
                self.presenter.core.handle_event(&x);
            }
        }
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
pub fn run(cfg: view::Config, session: crate::model::attempt::Session) -> Result<()> {
    let p = Presenter::new(presenter::Core::new(session));
    Manager::new(cfg)?.spawn(p)?.run()?;
    Ok(())
}
