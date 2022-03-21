//! Types used for interfacing with `ugly`.
//!
//! `ugly` is the backend `zombiesplit` uses to do text rendering and other such UI things.
//! It maintains fonts and colour palettes, and this module contains the particular identifiers we
//! use for those (as well as the default colour palettes).

pub mod colour;
pub mod font;

/// Shorthand for the type of renderer that `zombiesplit` uses.
pub trait Renderer: ugly::render::Renderer<font::Id, colour::fg::Id, colour::bg::Id> {}

impl<R: ugly::render::Renderer<font::Id, colour::fg::Id, colour::bg::Id>> Renderer for R {}
