//! The zombiesplit user interface.
pub mod error;
pub mod gfx;

pub use error::{Error, Result};
use sdl2::rect::Point;

/// The UI core.
pub struct Core {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
}

impl Core {
    /// Creates a new UI core.
    pub fn new() -> error::Result<Self> {
        let sdl = sdl2::init().map_err(Error::SdlInit)?;
        let video = sdl.video().map_err(Error::SdlInit)?;
        Ok(Core { sdl, video })
    }

    /// Runs the UI loop.
    pub fn run(&self, run: &crate::model::run::Run) -> error::Result<()> {
        // TODO(@MattWindsor91): pass in something other than Game.

        let window = gfx::make_window(&self.video)?;

        let mut screen = window.into_canvas().build().map_err(Error::SdlInteger)?;
        let textures = screen.texture_creator();

        let font = gfx::load_font(&textures, "font.png")?;

        screen.set_draw_color(sdl2::pixels::Color::BLACK);
        screen.clear();

        let mut r = gfx::Renderer { screen, font };

        let mut tl = Point::new(10, 10);
        for s in run.splits.iter() {
            r.put_str(&s.name, tl)?;
            tl = gfx::metrics::offset(tl, 0, 1);
        }
        r.present();

        let mut events = self.sdl.event_pump().map_err(Error::SdlInit)?;

        'running: loop {
            for event in events.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
