//! The low-level graphics rendering layer.

use std::{cell::RefMut, collections::HashMap, rc::Rc};

use super::{colour, metrics};
use crate::ui::error::{Error, Result};
use sdl2::{
    image::LoadTexture,
    rect::Point,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

/// The low-level graphics renderer.
pub struct Renderer<'a> {
    pub screen: RefMut<'a, Canvas<sdl2::video::Window>>,
    pub texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    pub textures: HashMap<FontId, Rc<Texture<'a>>>,
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
        }
    }

    /// Puts a string `str` onto the screen at `top_left`, using `font`.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    pub fn put_str(&mut self, str: &str, mut top_left: Point, font: FontId) -> Result<()> {
        let texture = self.font(font)?;
        for byte in str.as_bytes() {
            self.put_byte(&texture, *byte, top_left)?;
            top_left = metrics::offset(top_left, 1, 0);
        }
        Ok(())
    }

    fn put_byte<'b>(
        &'b mut self,
        texture: &'b Texture<'a>,
        byte: u8,
        top_left: Point,
    ) -> Result<()> {
        let src = metrics::font_rect(byte);
        let dst = metrics::char_rect(top_left);
        self.screen.copy(texture, src, dst).map_err(Error::Blit)
    }

    /// Clears the screen.
    pub fn clear(&mut self) {
        self.screen.set_draw_color(sdl2::pixels::Color::BLACK);
        self.screen.clear()
    }

    /// Refreshes the screen.
    pub fn present(&mut self) {
        self.screen.present()
    }

    /// Gets the font with id `id`, or loads it if it hasn't yet been loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if we need to load the font, but SDL cannot for some
    /// reason.
    pub fn font(&mut self, id: FontId) -> Result<Rc<Texture<'a>>> {
        self.textures.get(&id).cloned().map_or_else(
            || {
                let font = Rc::new(self.load_font(id)?);
                self.textures.insert(id, font.clone());
                Ok(font)
            },
            Ok,
        )
    }

    fn load_font(&mut self, id: FontId) -> Result<Texture<'a>> {
        match id {
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
}
