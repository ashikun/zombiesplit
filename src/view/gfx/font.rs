//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{collections::HashMap, fmt::Display, rc::Rc, str::FromStr};
use thiserror::Error;

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

use super::{colour, metrics};

/// A font manager, using a SDL texture creator.
pub struct Manager<'a> {
    /// The texture creator used to load fonts.
    creator: &'a TextureCreator<WindowContext>,
    /// The map of current font textures.
    textures: HashMap<(Id, colour::Key), Rc<Texture<'a>>>,
    /// The map of known font configurations.
    configs: &'a HashMap<Id, Config>,
}

impl<'a> Manager<'a> {
    /// Creates a font manager with the given texture creator and config hashmap.
    #[must_use]
    pub fn new(
        creator: &'a TextureCreator<WindowContext>,
        configs: &'a HashMap<Id, Config>,
    ) -> Self {
        Self {
            creator,
            textures: HashMap::new(),
            configs,
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
    ///
    /// # Errors
    ///
    /// Returns an error if the font is not configured.
    pub fn metrics(&self, id: Id) -> Result<metrics::Font> {
        self.config(id).map(|x| x.metrics)
    }

    fn config(&self, id: Id) -> Result<&Config> {
        self.configs.get(&id).ok_or(Error::Config(id))
    }

    fn cache(&mut self, id: Id, colour: colour::Key) -> Result<Rc<Texture<'a>>> {
        let tex = Rc::new(self.load(id, colour)?);
        self.textures.insert((id, colour), tex.clone());
        Ok(tex)
    }

    fn load(&mut self, id: Id, colour: colour::Key) -> Result<Texture<'a>> {
        let path = &self.config(id)?.path;
        let mut tex = self.creator.load_texture(path).map_err(Error::Load)?;
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
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, SerializeDisplay, DeserializeFromStr)]
pub enum Id {
    /// Normal font.
    Normal,
}

const NORMAL_STR: &str = "normal";

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => NORMAL_STR,
            }
        )
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            NORMAL_STR => Ok(Self::Normal),
            _ => Err(Error::Unknown(s.to_owned())),
        }
    }
}

/// A font configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The font path.
    path: String,
    /// The font metrics.
    metrics: metrics::Font,
}

/// A font error.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred while loading the font.
    #[error("couldn't load font: {0}")]
    Load(String),

    /// We tried to use a font configuration that doesn't exist.
    #[error("font not configured: {0}")]
    Config(Id),

    /// We tried to configure a font using a nonexistent ID.
    #[error("font id not recognised: {0}")]
    Unknown(String),
}

/// Shorthand for a result using [Error].
pub type Result<T> = std::result::Result<T, Error>;
