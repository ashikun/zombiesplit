//! Graphics rendering.

use super::error::{Error, Result};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};

/// The graphics renderer.
pub struct Renderer<'a> {
    // TODO(@MattWindsor91): hide these.
    pub screen: Canvas<sdl2::video::Window>,
    pub font: Texture<'a>,
}

const FONT_ROWS: u8 = 32;
const FONT_W: u8 = 7;
const FONT_H: u8 = 7;

/// Width of one character in the font, plus padding.
pub const FONT_WPAD: u8 = FONT_W + 1;
/// Height of one character in the font, plus padding.
pub const FONT_HPAD: u8 = FONT_H + 1;

impl<'a> Renderer<'a> {
    /// Puts a string [str] onto the screen at `top_left`.
    pub fn put_str(&mut self, str: &str, mut top_left: Point) -> Result<()> {
        for byte in str.as_bytes() {
            self.put_byte(*byte, top_left)?;
            top_left = top_left.offset(FONT_WPAD.into(), 0);
        }
        Ok(())
    }

    fn put_byte(&mut self, byte: u8, top_left: Point) -> Result<()> {
        let w32: u32 = FONT_W.into();
        let h32: u32 = FONT_H.into();

        let wpad: i32 = FONT_WPAD.into();
        let hpad: i32 = FONT_HPAD.into();

        let row : i32 = (byte % FONT_ROWS).into();
        let col : i32 = (byte / FONT_ROWS).into();
        let src = Rect::new(
            row * wpad,
            col * hpad,
            w32,
            h32);
        let dst = 
        Rect::new(
            top_left.x,
            top_left.y,
            w32,
            h32);
        self.screen.copy(&self.font, src, dst).map_err(Error::SdlBlit)
    }
}

/// Makes a zombiesplit window.
pub fn make_window(video: &sdl2::VideoSubsystem) -> Result<sdl2::video::Window> {
    let window = video.window("zombiesplit", 320, 640)
        .position_centered()
        .build()
        .map_err(Error::SdlWindow)?;
    Ok(window)
}
