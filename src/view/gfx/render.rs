//! The low-level graphics rendering layer.

use std::{cell::RefMut, collections::HashMap, rc::Rc};

use super::super::error::{Error, Result};
use super::{colour, metrics, position::Position};
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video,
};

/// Trait of things that provide rendering facilities.
pub trait Renderer {
    /// Sets the plotter to the given position.
    fn set_pos(&mut self, pos: Position);

    /// Moves the plotter by the given number of characters in the current font.
    fn move_chars(&mut self, dx: i32, dy: i32);

    /// Sets the current font.
    fn set_font(&mut self, font: FontId);

    /// Puts a string `str` onto the screen at the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    fn put_str(&mut self, str: &str) -> Result<()>;
}

/// The low-level window graphics renderer.
pub struct Window<'a> {
    screen: RefMut<'a, Canvas<video::Window>>,
    texture_creator: &'a TextureCreator<video::WindowContext>,
    textures: HashMap<FontId, Rc<Texture<'a>>>,

    /// The current font.
    font: FontId,
    /// The current font's metrics.
    fmetrics: metrics::Font,
    /// The current position.
    pos: Point,
}

/// Font IDs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontId {
    /// The normal font, given a particular colour.
    Normal(colour::Key),
}

impl<'a> Renderer for Window<'a> {
    /// Sets the plotter to the given position.
    fn set_pos(&mut self, pos: Position) {
        self.pos = Point::new(
            pos.x.to_left(self.pos.x, metrics::WINDOW.win_w),
            pos.y.to_top(self.pos.y, metrics::WINDOW.win_h),
        )
    }

    /// Moves the plotter by the given number of characters in the current font.
    fn move_chars(&mut self, dx: i32, dy: i32) {
        self.set_pos(Position::rel(
            self.fmetrics.span_w(dx),
            self.fmetrics.span_h(dy),
        ))
    }

    /// Sets the current font.
    fn set_font(&mut self, font: FontId) {
        self.font = font
    }

    /// Puts a string `str` onto the screen at the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
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
    pub fn new(
        screen: RefMut<'a, Canvas<video::Window>>,
        texture_creator: &'a TextureCreator<video::WindowContext>,
    ) -> Self {
        Self {
            screen,
            texture_creator,
            textures: HashMap::new(),
            font: FontId::Normal(colour::Key::NoTime),
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

    /// Gets the current font as a texture, or loads it if it hasn't yet been
    /// loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if we need to load the font, but SDL cannot for some
    /// reason.
    fn font_texture(&mut self) -> Result<Rc<Texture<'a>>> {
        self.textures
            .get(&self.font)
            .cloned()
            .map_or_else(|| self.cache_font(), Ok)
    }

    fn cache_font(&mut self) -> Result<Rc<Texture<'a>>> {
        let tex = Rc::new(self.load_font(self.font)?);
        self.textures.insert(self.font, tex.clone());
        Ok(tex)
    }

    fn load_font(&mut self, id: FontId) -> Result<Texture<'a>> {
        // TODO(@MattWindsor91): make a proper resource manager.
        match id {
            // TODO(@MattWindsor91): separate font and colour keys.
            FontId::Normal(ckey) => {
                let mut font = self
                    .texture_creator
                    .load_texture("font.png")
                    .map_err(Error::LoadFont)?;
                let colour = colour::SET.by_key(ckey);
                font.set_color_mod(colour.r, colour.g, colour.b);
                font.set_alpha_mod(colour.a);
                Ok(font)
            }
        }
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
    fn set_font(&mut self, font: FontId) {
        self.renderer.set_font(font)
    }
    fn put_str(&mut self, str: &str) -> Result<()> {
        self.renderer.put_str(str)
    }
}
