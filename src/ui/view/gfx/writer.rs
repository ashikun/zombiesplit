//! The [Writer] struct.

use super::{font, metrics};

/// Helper for positioned writing of strings.
pub struct Writer<'r, R: ?Sized> {
    /// The point used as the anchor for the writing.
    pos: metrics::Point,

    /// The alignment for the writing.
    alignment: metrics::anchor::X,

    /// The specification of the font being used for writing.
    font_spec: font::Spec,

    /// Reference to the renderer being borrowed to do the rendering.
    renderer: &'r mut R,
}

impl<'r, R: super::Renderer + ?Sized> Writer<'r, R> {
    /// Constructs a writer on `renderer`, using the font spec `font_spec`.
    ///
    /// The writer initially points to the origin and uses a left anchor.
    pub fn new(renderer: &'r mut R) -> Self {
        Self {
            renderer,
            font_spec: font::Spec::default(),
            pos: metrics::Point::default(),
            alignment: metrics::anchor::X::Left,
        }
    }

    pub fn with_font(mut self, font_spec: font::Spec) -> Self {
        self.font_spec = font_spec;
        self
    }

    /// Moves the writer to position `pos`.
    pub fn with_pos(mut self, pos: metrics::Point) -> Self {
        self.pos = pos;
        self
    }
}

/// We can use writers with Rust's formatting system.
impl<'r, R: super::Renderer + ?Sized> std::fmt::Write for Writer<'r, R> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        // TODO(@MattWindsor91): compute position according to anchor
        self.renderer
            .write(self.pos, self.font_spec, s)
            .map_err(|_| std::fmt::Error)
    }
}
