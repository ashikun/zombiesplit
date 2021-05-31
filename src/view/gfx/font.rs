//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

use std::{collections::HashMap, rc::Rc};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use crate::view::error::{Error, Result};

use super::colour;

/// A font manager, using a SDL texture creator.
pub struct Manager<'a> {
    /// The texture creator used to load fonts.
    creator: &'a TextureCreator<WindowContext>,
    /// The map of current font textures.
    textures: HashMap<(Id, colour::Key), Rc<Texture<'a>>>,
}

impl<'a> Manager<'a> {
    #[must_use]
    pub fn new(creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            creator,
            textures: HashMap::new(),
        }
    }

    /// Gets the given font (with the given colour) as a texture, or loads it if
    /// it hasn't yet been loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if we need to load the font, but SDL cannot for some
    /// reason.
    pub fn font_texture(&mut self, id: Id, colour: colour::Key) -> Result<Rc<Texture<'a>>> {
        self.textures
            .get(&(id, colour))
            .cloned()
            .map_or_else(|| self.cache_font(id, colour), Ok)
    }

    fn cache_font(&mut self, id: Id, colour: colour::Key) -> Result<Rc<Texture<'a>>> {
        let tex = Rc::new(self.load_font(id, colour)?);
        self.textures.insert((id, colour), tex.clone());
        Ok(tex)
    }

    fn load_font(&mut self, id: Id, colour: colour::Key) -> Result<Texture<'a>> {
        let mut tex = self
            .creator
            .load_texture(id.filename())
            .map_err(Error::LoadFont)?;
        colourise(&mut tex, colour);
        Ok(tex)
    }
}

fn colourise(texture: &mut Texture, colour: colour::Key) {
    // TODO(@MattWindsor91): decouple colour::SET
    let colour = colour::SET.by_key(colour);
    texture.set_color_mod(colour.r, colour.g, colour.b);
    texture.set_alpha_mod(colour.a);
}

/// A key in the font manager's lookup table.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Id {
    /// Normal font.
    Normal,
}

impl Id {
    fn filename(self) -> &'static str {
        // TODO(@MattWindsor91): de-hardcode this.
        match self {
            Self::Normal => "font.png",
        }
    }
}
