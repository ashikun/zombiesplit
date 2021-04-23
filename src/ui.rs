//! The zombiesplit user interface.
pub mod error;
pub mod gfx;

pub use error::{Error, Result};
use sdl2::rect::Point;

use self::gfx::Renderer;
use crate::model::run;

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

        self.draw_splits(run.splits.iter(), &mut r)?;

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

    fn draw_splits<'a>(
        &self,
        splits: impl IntoIterator<Item = &'a run::Split>,
        r: &mut Renderer,
    ) -> Result<()> {
        for (num, s) in splits.into_iter().enumerate() {
            self.draw_split(s, num, r)?;
        }
        Ok(())
    }

    fn draw_split(&self, split: &run::Split, num: usize, r: &mut Renderer) -> Result<()> {
        let tl = Point::new(4, 4 + (16 * num as i32));
        r.put_str(&split.name, tl)
    }
}
