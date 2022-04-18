//! Types used for interfacing with `ugly`.
//!
//! `ugly` is the backend `zombiesplit` uses to do text rendering and other such UI things.
//! It maintains fonts and colour palettes, and this module contains the particular identifiers we
//! use for those (as well as the default colour palettes).

pub mod colour;
pub mod font;

/// Shorthand for the type of renderer that `zombiesplit` uses.
pub trait Renderer:
    ugly::render::Renderer<font::Map<ugly::Font>, colour::fg::Map, colour::bg::Map>
{
}

impl<R: ugly::render::Renderer<font::Map<ugly::Font>, colour::fg::Map, colour::bg::Map>> Renderer
    for R
{
}
