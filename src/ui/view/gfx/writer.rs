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

/// We can use writers as if they were I/O writers.
impl<'r, R: super::Renderer + ?Sized> std::io::Write for Writer<'r, R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // TODO(@MattWindsor91): compute position according to anchor
        self.renderer
            .write(self.pos, self.font_spec, buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
