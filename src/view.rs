//! The zombiesplit user interface.
pub mod error;
mod event;
pub mod gfx;

use crate::presenter;
use std::cell::RefCell;

pub use error::{Error, Result};

use crate::model::run;

/// A UI manager, owning the various resources the UI core uses.
pub struct Manager {
    sdl: sdl2::Sdl,
    screen: RefCell<sdl2::render::Canvas<sdl2::video::Window>>,
    textures: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
}

impl Manager {
    /// Creates a new UI manager, opening a window in the process.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the SDL subsystems the UI manager requires
    /// fail to initialise.
    pub fn new() -> Result<Self> {
        let sdl = sdl2::init().map_err(Error::Init)?;
        let video = sdl.video().map_err(Error::Init)?;
        let window = gfx::make_window(&video)?;
        let screen = window.into_canvas().build().map_err(Error::SdlInteger)?;
        let textures = screen.texture_creator();
        Ok(Self {
            sdl,
            screen: RefCell::new(screen),
            textures,
        })
    }

    /// Spawns a [Core] handling UI services.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL can't spawn an event pump.
    pub fn spawn(&self, r: run::Run) -> Result<Core> {
        let renderer = gfx::render::Window::new(self.screen.borrow_mut(), &self.textures);
        let gfx = gfx::Core::new(renderer);
        let state = presenter::Presenter::new(r);

        let events = self.sdl.event_pump().map_err(Error::Init)?;

        Ok(Core { events, gfx, state })
    }
}

/// The UI core.
pub struct Core<'a> {
    events: sdl2::EventPump,
    gfx: gfx::Core<'a>,
    state: presenter::Presenter,
}

impl<'a> Core<'a> {
    /// Runs the UI loop.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to perform an action.
    pub fn run(&mut self) -> error::Result<()> {
        // TODO(@MattWindsor91): pass in something other than Game.

        self.gfx.redraw(&self.state)?;

        while self.state.is_running() {
            for e in self.events.poll_iter() {
                if let Some(x) = event::from_sdl(&e) {
                    self.state.handle_event(&x)
                }
            }
            self.redraw_if_dirty()?;
        }

        Ok(())
    }

    fn redraw_if_dirty(&mut self) -> error::Result<()> {
        if self.state.is_dirty {
            self.gfx.redraw(&self.state)
        } else {
            Ok(())
        }
    }
}
