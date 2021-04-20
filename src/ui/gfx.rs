//! Graphics rendering.

pub mod metrics; // for now

use super::error::{Error, Result};
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture};

/// The graphics renderer.
pub struct Renderer<'a> {
    // TODO(@MattWindsor91): hide these.
    pub screen: Canvas<sdl2::video::Window>,
    pub font: Texture<'a>,
}

impl<'a> Renderer<'a> {
    /// Puts a string [str] onto the screen at `top_left`.
    pub fn put_str(&mut self, str: &str, mut top_left: Point) -> Result<()> {
        for byte in str.as_bytes() {
            self.put_byte(*byte, top_left)?;
            top_left = metrics::offset(top_left, 1, 0);
        }
        Ok(())
    }

    fn put_byte(&mut self, byte: u8, top_left: Point) -> Result<()> {
        let src = metrics::font_rect(byte);
        let dst = metrics::char_rect(top_left);
        self.screen
            .copy(&self.font, src, dst)
            .map_err(Error::SdlBlit)
    }
}

/// Makes a zombiesplit window.
pub fn make_window(video: &sdl2::VideoSubsystem) -> Result<sdl2::video::Window> {
    let window = video
        .window("zombiesplit", 320, 640)
        .position_centered()
        .build()
        .map_err(Error::SdlWindow)?;
    Ok(window)
}
