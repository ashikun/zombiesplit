//! The low-level graphics rendering layer.

use std::{cell::RefMut, collections::HashMap, rc::Rc};

use super::{colour, metrics};
use crate::ui::error::{Error, Result};
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

/// The low-level graphics renderer.
pub struct Renderer<'a> {
    screen: RefMut<'a, Canvas<sdl2::video::Window>>,
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
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

impl<'a> Renderer<'a> {
    /// Constructs a [Renderer] using the given screen and texture creator.
    #[must_use]
    pub fn new(
        screen: RefMut<'a, Canvas<Window>>,
        texture_creator: &'a TextureCreator<WindowContext>,
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

    /// Sets the plotter to the given position.
    pub fn set_pos(&mut self, x: i32, y: i32) -> &mut Self {
        self.pos = Point::new(x, y);
        self
    }

    /// Moves the plotter by the given pixel deltas.
    pub fn move_pos(&mut self, dx: i32, dy: i32) -> &mut Self {
        self.pos = self.pos.offset(dx, dy);
        self
    }

    /// Sets the plotter to the given horizontal position.
    pub fn set_x(&mut self, x: i32) -> &mut Self {
        self.set_pos(x, self.pos.y)
    }

    /// Moves the plotter by the given number of characters in the current font.
    pub fn move_chars(&mut self, dx: i32, dy: i32) -> &mut Self {
        self.move_pos(self.fmetrics.span_w(dx), self.fmetrics.span_h(dy))
    }

    /// Sets the current font.
    pub fn set_font(&mut self, font: FontId) -> &mut Self {
        self.font = font;
        self
    }

    /// Clears the screen.
    pub fn clear(&mut self) -> &mut Self {
        self.screen.set_draw_color(colour::SET.bg);
        self.screen.clear();
        self
    }

    /// Refreshes the screen.
    pub fn present(&mut self) -> &mut Self {
        self.screen.present();
        self
    }

    /// Puts a string `str` onto the screen at the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    pub fn put_str(&mut self, str: &str) -> Result<&mut Self> {
        let old_pos = self.pos;
        let texture = self.font_texture()?;

        for byte in str.as_bytes() {
            self.put_byte(&texture, *byte, self.pos)?;
            self.move_chars(1, 0);
        }

        self.pos = old_pos;
        Ok(self)
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
