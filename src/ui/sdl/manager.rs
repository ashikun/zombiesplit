//! The [Manager] struct.

use std::cell::RefCell;

use super::{
    super::{
        error::{Error, Result},
        view,
    },
    event, font, metrics, Renderer,
};

/// Manages top-level SDL resources.
pub struct Manager<'c> {
    sdl: sdl2::Sdl,
    screen: RefCell<sdl2::render::Canvas<sdl2::video::Window>>,
    textures: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    /// The configured theme (fonts and colours), which is borrowing parts of a config file.
    cfg: &'c view::config::theme::Theme,
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
        let window = make_window(&video, cfg.layout.window)?;
        let screen = window.into_canvas().build().map_err(Error::SdlInteger)?;
        let textures = screen.texture_creator();
        Ok(Self {
            sdl,
            screen: RefCell::new(screen),
            textures,
            cfg: &cfg.theme,
        })
    }
}

impl<'r, 'c> super::super::Manager<'r> for Manager<'c> {
    type Pump = event::Pump;
    type Renderer = Renderer<'r>;

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
    fn renderer(&'r self) -> Result<Renderer<'r>> {
        let font_manager = font::Manager::new(
            &self.textures,
            &self.cfg.font_paths,
            self.cfg.font_metrics.clone(),
            &self.cfg.colours.fg,
        );
        Ok(Renderer::new(
            self.screen.borrow_mut(),
            font_manager,
            &self.cfg.colours,
        ))
    }
}

/// Makes a zombiesplit window.
///
/// # Errors
///
/// Returns an error if SDL fails to make the window.
fn make_window(
    video: &sdl2::VideoSubsystem,
    wmetrics: view::gfx::metrics::Window,
) -> Result<sdl2::video::Window> {
    let window = video
        .window(
            "zombiesplit",
            metrics::u32_or_zero(wmetrics.win_w),
            metrics::u32_or_zero(wmetrics.win_h),
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
