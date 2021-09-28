//! Traits for low-level rendering.

use super::{super::super::Result, colour, font, metrics, position::Position};

/// Trait of things that provide rendering facilities.
pub trait Renderer: font::metrics::TextSizer {
    /// Gets the size of this renderer's bounding box.
    fn size(&self) -> metrics::Size;

    /// Sets the plotter to the given position.
    fn set_pos(&mut self, pos: Position);

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
}

/// A renderer that delegates to an underlying renderer, but maps coordinates
/// into a fenced region.
pub struct Region<'a> {
    /// The underlying renderer.
    renderer: &'a mut dyn Renderer,
    /// The bounding box, relative to the parent renderer.
    rect: metrics::Rect,
}

impl<'a> Region<'a> {
    /// Constructs a new [Region] using the given renderer and bounding box.    
    pub fn new(renderer: &'a mut dyn Renderer, rect: metrics::Rect) -> Self {
        Self { renderer, rect }
    }
}

impl<'a> Renderer for Region<'a> {
    fn size(&self) -> metrics::Size {
        self.rect.size
    }

    fn set_pos(&mut self, pos: Position) {
        self.renderer.set_pos(pos.normalise_to_rect(self.rect));
    }
    fn set_font(&mut self, font: font::Id) {
        self.renderer.set_font(font);
    }
    fn set_bg_colour(&mut self, colour: colour::bg::Id) {
        self.renderer.set_bg_colour(colour);
    }
    fn set_fg_colour(&mut self, colour: colour::fg::Id) {
        self.renderer.set_fg_colour(colour);
    }
    fn put_str(&mut self, str: &str) -> Result<()> {
        self.renderer.put_str(str)
    }
    fn put_str_r(&mut self, str: &str) -> Result<()> {
        self.renderer.put_str_r(str)
    }
    fn fill(&mut self, rect: metrics::Rect) -> Result<()> {
        self.renderer.fill(rect)
    }

    fn clear(&mut self) {
        // TODO(@MattWindsor91): this is incorrect, but we'll be removing Region anyway.
        self.renderer.clear();
    }

    fn present(&mut self) {
        // TODO(@MattWindsor91): this is incorrect, but we'll be removing Region anyway.
        self.renderer.clear();
    }
}

/// We delegate text sizing to the next renderer in the chain.
impl<'a> font::metrics::TextSizer for Region<'a> {
    fn span_w(&self, size: i32) -> i32 {
        self.renderer.span_w(size)
    }

    fn span_h(&self, size: i32) -> i32 {
        self.renderer.span_h(size)
    }
}
