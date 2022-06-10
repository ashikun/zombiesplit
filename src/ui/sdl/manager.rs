//! The [Manager] struct.

use super::{
    super::{
        error::{Error, Result},
        view,
    },
    event,
};

/// Manages top-level SDL resources.
pub struct Manager<'c> {
    sdl: sdl2::Sdl,
    gfx: ugly::backends::sdl::Manager<
        'c,
        view::gfx::font::Map<ugly::Font>,
        view::gfx::colour::fg::Map,
        view::gfx::colour::bg::Map,
        sdl2::video::Window,
    >,
}

impl<'c> Manager<'c> {
    /// Creates a new view, opening a window in the process.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the SDL subsystems the UI manager requires
    /// fail to initialise.
    pub fn new(cfg: &'c view::Config) -> Result<Self> {
        let sdl = sdl2::init().map_err(Error::Init)?;
        let video = sdl.video().map_err(Error::Init)?;
        let window = make_window(&video, cfg.layout.window.size)?;
        let gfx =
            ugly::backends::sdl::Manager::new(window, &cfg.theme.font_paths, &cfg.theme.colours)?;
        Ok(Self { sdl, gfx })
    }
}

impl<'r, 'c> super::super::Manager<'r> for Manager<'c> {
    type Pump = event::Pump;
    type Renderer = ugly::backends::sdl::Renderer<
        'r,
        view::gfx::font::Map<ugly::Font>,
        view::gfx::colour::fg::Map,
        view::gfx::colour::bg::Map,
        sdl2::video::Window,
    >;

    /// Spawns an event pump using the SDL event pump.
    ///
    /// # Errors
    ///
    /// Fails if we can't get access to the event pump.
    fn event_pump(&self) -> Result<event::Pump> {
        self.sdl
            .event_pump()
            .map(event::Pump::new)
            .map_err(Error::Init)
    }

    /// Spawns a renderer targeting the SDL window.
    ///
    /// # Errors
    ///
    /// Returns an error if the font metrics are nonsensical.
    fn renderer(&'r self) -> Result<Self::Renderer> {
        Ok(self.gfx.renderer()?)
    }
}

/// Makes a zombiesplit window.
///
/// # Errors
///
/// Returns an error if SDL fails to make the window.
#[allow(clippy::cast_sign_loss)]
fn make_window(
    video: &sdl2::VideoSubsystem,
    size: ugly::metrics::Size,
) -> Result<sdl2::video::Window> {
    // TODO(@MattWindsor91): move this into ugly.
    let w = size.w.max(0) as u32;
    let h = size.h.max(0) as u32;
    let window = video
        .window("zombiesplit", w, h)
        .position_centered()
        .resizable()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
