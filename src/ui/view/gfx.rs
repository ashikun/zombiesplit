//! Types used for interfacing with `ugly`.
//!
//! `ugly` is the backend `zombiesplit` uses to do text rendering and other such UI things.
//! It maintains fonts and colour palettes, and this module contains the particular identifiers we
//! use for those (as well as the default colour palettes).

pub mod colour;
pub mod font;

/// Shorthand for the type of renderer that `zombiesplit` uses.
pub trait Renderer<'a>:
    ugly::render::Renderer<'a, font::Map<ugly::Font>, colour::fg::Map, colour::bg::Map>
{
}

impl<
        'a,
        R: ugly::render::Renderer<'a, font::Map<ugly::Font>, colour::fg::Map, colour::bg::Map>,
    > Renderer<'a> for R
{
}

/// Shorthand for the writer type over zombiesplit's resource maps.
pub type Writer = ugly::text::Writer<font::Map<ugly::Font>, colour::fg::Map, colour::bg::Map>;
