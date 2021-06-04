//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

pub mod error;
pub mod metrics;
pub use error::{Error, Result};
pub use metrics::Metrics;

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, rc::Rc};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use super::colour;

/// A font manager, using a SDL texture creator.
pub struct Manager<'a> {
    /// The texture creator used to load fonts.
    creator: &'a TextureCreator<WindowContext>,
    /// The map of current font textures.
    textures: HashMap<(Id, colour::Key), Rc<Texture<'a>>>,
    /// The font set, containing configuration for each font.
    font_set: &'a Set,
    /// The foreground colour set, used for setting up font colours.
    colour_set: &'a colour::ForegroundSet,
}

impl<'a> Manager<'a> {
    /// Creates a font manager with the given texture creator and config hashmap.
    #[must_use]
    pub fn new(
        creator: &'a TextureCreator<WindowContext>,
        font_set: &'a Set,
        colour_set: &'a colour::ForegroundSet,
    ) -> Self {
        Self {
            creator,
            textures: HashMap::new(),
            font_set,
            colour_set,
        }
    }

    /// Gets the given font (with the given colour) as a texture, or loads it if
    /// it hasn't yet been loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if we need to load the font but SDL cannot for some
    /// reason, or the font is not configured.
    pub fn texture(&mut self, id: Id, colour: colour::Key) -> Result<Rc<Texture<'a>>> {
        self.textures
            .get(&(id, colour))
            .cloned()
            .map_or_else(|| self.cache(id, colour), Ok)
    }

    /// Gets the given font's metrics set.
    #[must_use]
    pub fn metrics(&self, id: Id) -> Metrics {
        self.font_set.get(id).metrics
    }

    fn cache(&mut self, id: Id, colour: colour::Key) -> Result<Rc<Texture<'a>>> {
        let tex = Rc::new(self.load(id, colour)?);
        self.textures.insert((id, colour), tex.clone());
        Ok(tex)
    }

    fn load(&mut self, id: Id, colour: colour::Key) -> Result<Texture<'a>> {
        let path = &self.font_set.get(id).path;
        let mut tex = self.creator.load_texture(path).map_err(Error::Load)?;
        self.colourise(&mut tex, colour);
        Ok(tex)
    }

    fn colourise(&self, texture: &mut Texture, colour: colour::Key) {
        // TODO(@MattWindsor91): decouple colour::SET
        let colour = sdl2::pixels::Color::from(self.colour_set.by_key(colour));
        texture.set_color_mod(colour.r, colour.g, colour.b);
        texture.set_alpha_mod(colour.a);
    }
}

/// A key in the font manager's lookup table.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Id {
    /// Normal font.
    Normal,
    /// Large font.
    Large,
}

/// A font configuration set.
#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    /// The normal font.
    pub normal: Config,
    /// The large font, used for titles and totals.
    pub large: Config,
}

impl Set {
    /// Gets the configuration for a particular font.
    #[must_use]
    pub fn get(&self, id: Id) -> &Config {
        match id {
            Id::Normal => &self.normal,
            Id::Large => &self.large,
        }
    }
}

/// A font configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The font path.
    pub path: String,
    /// The font metrics.
    pub metrics: Metrics,
}
