//! The low-level graphics rendering layer.

use std::{cell::RefMut, rc::Rc};

use super::super::error::{Error, Result};
use super::{colour, font, metrics, position::Position};
use sdl2::{
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video,
};

/// Trait of things that provide rendering facilities.
pub trait Renderer {
    /// Sets the plotter to the given position.
    fn set_pos(&mut self, pos: Position);

    /// Moves the plotter by the given number of characters in the current font.
    fn move_chars(&mut self, dx: i32, dy: i32);

    /// Sets the current font.
    fn set_font(&mut self, font: font::Id);

    /// Sets the current foreground colour.
    fn set_fg_colour(&mut self, colour: colour::Key);

    /// Puts a string `str` onto the screen at the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    fn put_str(&mut self, str: &str) -> Result<()>;

    /// Puts a string `str` onto the screen with the right side positioned at
    /// the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    fn put_str_r(&mut self, str: &str) -> Result<()> {
        let len = metrics::sat_i32(str.len());
        self.move_chars(-len, 0);
        self.put_str(str)?;
        self.move_chars(len, 0);
        Ok(())
    }
}

/// The low-level window graphics renderer.
pub struct Window<'a> {
    screen: RefMut<'a, Canvas<video::Window>>,
    font_manager: font::Manager<'a>,

    /// The current font.
    font: font::Id,
    /// The current font colour.
    colour: colour::Key,
    /// The current font's metrics.
    fmetrics: metrics::Font,
    /// The current position.
    pos: Point,
}

impl<'a> Renderer for Window<'a> {
    fn set_pos(&mut self, pos: Position) {
        self.pos = Point::new(
            pos.x.to_left(self.pos.x, metrics::WINDOW.win_w),
            pos.y.to_top(self.pos.y, metrics::WINDOW.win_h),
        )
    }

    fn move_chars(&mut self, dx: i32, dy: i32) {
        self.set_pos(Position::rel(
            self.fmetrics.span_w(dx),
            self.fmetrics.span_h(dy),
        ))
    }

    fn set_font(&mut self, font: font::Id) {
        self.font = font
    }

    fn set_fg_colour(&mut self, colour: colour::Key) {
        self.colour = colour
    }

    fn put_str(&mut self, str: &str) -> Result<()> {
        let old_pos = self.pos;
        let texture = self.font_texture()?;

        for byte in str.as_bytes() {
            self.put_byte(&texture, *byte, self.pos)?;
            self.move_chars(1, 0);
        }

        self.pos = old_pos;
        Ok(())
    }
}

impl<'a> Window<'a> {
    /// Constructs a [Renderer] using the given screen and texture creator.
    #[must_use]
    pub fn new(screen: RefMut<'a, Canvas<video::Window>>, font_manager: font::Manager<'a>) -> Self {
        Self {
            screen,
            font_manager,
            font: font::Id::Normal,
            colour: colour::Key::NoTime,
            fmetrics: metrics::FONT,
            pos: Point::new(0, 0),
        }
    }

    /// Clears the screen.
    pub fn clear(&mut self) {
        self.screen.set_draw_color(colour::SET.bg);
        self.screen.clear()
    }

    /// Refreshes the screen.
    pub fn present(&mut self) {
        self.screen.present()
    }

    fn font_texture(&mut self) -> Result<Rc<Texture<'a>>> {
        self.font_manager.font_texture(self.font, self.colour)
    }

    fn put_byte<'b>(
        &'b mut self,
        texture: &'b Texture<'a>,
        byte: u8,
        top_left: Point,
    ) -> Result<()> {
        let src = self.font_rect(byte);
        let dst = self.char_rect(top_left);
        self.screen.copy(texture, src, dst).map_err(Error::Blit)
    }

    /// Produces a rectangle with top-left `top_left` and the size of one font
    /// character.
    #[must_use]
    fn char_rect(&self, top_left: Point) -> Rect {
        let w = self.fmetrics.char_w;
        let h = self.fmetrics.char_h;
        Rect::new(top_left.x, top_left.y, u32::from(w), u32::from(h))
    }

    /// Produces the appropriate rectangle for looking up `char` in the font.
    #[must_use]
    fn font_rect(&self, char: u8) -> Rect {
        self.char_rect(Point::new(
            self.fmetrics.glyph_x(char),
            self.fmetrics.glyph_y(char),
        ))
    }
}

/// A renderer that delegates to an underlying renderer, but maps coordinates
/// into a fenced region.
pub struct Region<'a> {
    /// The underlying renderer.
    renderer: &'a mut dyn Renderer,
    /// The bounding box, relative to the parent renderer.
    rect: metrics::Rect,
}

impl<'a> Region<'a> {
    /// Constructs a new [Region] using the given renderer and bounding box.    
    pub fn new(renderer: &'a mut dyn Renderer, rect: metrics::Rect) -> Self {
        Self { renderer, rect }
    }
}

impl<'a> Renderer for Region<'a> {
    fn set_pos(&mut self, pos: Position) {
        self.renderer.set_pos(pos.normalise_to_rect(self.rect))
    }
    fn move_chars(&mut self, dx: i32, dy: i32) {
        self.renderer.move_chars(dx, dy)
    }
    fn set_font(&mut self, font: font::Id) {
        self.renderer.set_font(font)
    }
    fn set_fg_colour(&mut self, colour: colour::Key) {
        self.renderer.set_fg_colour(colour)
    }
    fn put_str(&mut self, str: &str) -> Result<()> {
        self.renderer.put_str(str)
    }
}
