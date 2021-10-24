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
    /// # Errors
    ///
    /// Fails if the renderer can't render the writing.
    fn write(&mut self, pos: Point, font: font::Spec, s: &str) -> Result<()>;

    /// Sets the plotter to the given position.
    fn set_pos(&mut self, pos: Point);

    /// Sets the current font.
    fn set_font(&mut self, font: font::Id);

    /// Sets the current background colour.
    fn set_bg_colour(&mut self, colour: colour::bg::Id);

    /// Sets the current foreground colour.
    fn set_fg_colour(&mut self, colour: colour::fg::Id);

    /// Sets both colours at the same time.
    fn set_colours(&mut self, fg: colour::fg::Id, bg: colour::bg::Id) {
        self.set_fg_colour(fg);
        self.set_bg_colour(bg);
    }

    /// Puts a string `str` onto the screen at the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    fn put_str(&mut self, str: &str) -> Result<()>;

    /// Puts a string `str` onto the screen with the right side positioned at
    /// the current coordinate.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to load the font (if it has not been
    /// loaded already), or fails to blit the font onto the screen.
    fn put_str_r(&mut self, str: &str) -> Result<()>;

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
