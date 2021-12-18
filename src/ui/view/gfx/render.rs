//! Traits for low-level rendering.

use super::{
    colour, font,
    metrics::{self, Point},
    Result,
};

/// Trait of things that provide rendering facilities.
pub trait Renderer {
    /// Writes the string `s` at position `pos` with the font `font`.
    ///
    /// Returns the position that the next character would be written to, if we continued writing.
    ///
    /// # Errors
    ///
    /// Fails if the renderer can't render the writing.
    fn write(&mut self, pos: Point, font: font::Spec, s: &str) -> Result<Point>;

    /// Sets the current background colour.
    fn set_bg_colour(&mut self, colour: colour::bg::Id);

    /// Fills the rectangle `rect`, whose top-left is positioned relative to
    /// the current position, with the current background colour.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to blit the rect onto the screen.
    fn fill(&mut self, rect: metrics::Rect) -> Result<()>;

    // TODO(@MattWindsor91): replace these with RAII

    /// Clears the screen.
    fn clear(&mut self);

    /// Refreshes the screen.
    fn present(&mut self);

    // TODO(@MattWindsor91): make the below obsolete?

    /// Borrows the font metrics map.
    fn font_metrics(&self) -> &font::Map<font::Metrics>;
}
