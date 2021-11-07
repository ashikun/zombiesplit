//! The font manager.
use std::{collections::HashMap, rc::Rc};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use super::super::view::gfx::{
    colour,
    font::{self, map::Spec, Error, Result},
};

/// A font manager, using a SDL texture creator.
pub struct Manager<'a> {
    /// The texture creator used to load fonts.
    creator: &'a TextureCreator<WindowContext>,
    /// The map of current font textures.
    textures: HashMap<Spec, Rc<Texture<'a>>>,
    /// The font path set.
    font_set: font::Map<font::map::Path<'a>>,
    /// The font metrics set.
    pub metrics_set: font::Map<font::Metrics>,
    /// The foreground colour set, used for setting up font colours.
    colour_set: &'a colour::fg::Set,
}

impl<'a> Manager<'a> {
    /// Creates a font manager with the given texture creator and config maps.
    #[must_use]
    pub fn new(
        creator: &'a TextureCreator<WindowContext>,
        font_set: font::Map<font::map::Path<'a>>,
        metrics_set: font::Map<font::Metrics>,
        colour_set: &'a colour::fg::Set,
    ) -> Self {
        Self {
            creator,
            textures: HashMap::new(),
            font_set,
            metrics_set,
            colour_set,
        }
    }

    /// Gets the given font spec as a texture, or loads it if
    /// it hasn't yet been loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if we need to load the font but SDL cannot for some
    /// reason, or the font is not configured.
    pub fn texture(&mut self, spec: Spec) -> Result<Rc<Texture<'a>>> {
        self.textures
            .get(&spec)
            .cloned()
            .map_or_else(|| self.cache(spec), Ok)
    }

    fn cache(&mut self, spec: Spec) -> Result<Rc<Texture<'a>>> {
        let tex = Rc::new(self.load(spec)?);
        self.textures.insert(spec, tex.clone());
        Ok(tex)
    }

    fn load(&mut self, spec: Spec) -> Result<Texture<'a>> {
        let path = &self.font_set[spec.id].texture_path();
        let mut tex = self
            .creator
            .load_texture(path)
            .map_err(Error::TextureLoad)?;
        self.colourise(&mut tex, spec.colour);
        Ok(tex)
    }

    fn colourise(&self, texture: &mut Texture, colour: colour::fg::Id) {
        let colour = self.colour_set.get(colour);
        texture.set_color_mod(colour.red_byte(), colour.green_byte(), colour.blue_byte());
        texture.set_alpha_mod(colour.alpha_byte());
    }
}
