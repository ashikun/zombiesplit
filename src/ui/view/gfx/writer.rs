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

    font_metrics: font::Metrics,

    /// Reference to the renderer being borrowed to do the rendering.
    renderer: &'r mut R,
}

impl<'r, R: super::Renderer + ?Sized> Writer<'r, R> {
    /// Constructs a writer on `renderer`, using the font spec `font_spec`.
    ///
    /// The writer initially points to the origin and uses a left anchor.
    pub fn new(renderer: &'r mut R) -> Self {
        let font_spec = font::Spec::default();
        let font_metrics = renderer.font_metrics()[font_spec.id].clone();
        Self {
            renderer,
            font_spec: font::Spec::default(),
            font_metrics,
            pos: metrics::Point::default(),
            alignment: metrics::anchor::X::Left,
        }
    }

    /// Changes the writer to use font `font_spec`.
    #[must_use]
    pub fn with_font(mut self, font_spec: font::Spec) -> Self {
        self.font_spec = font_spec;
        self.font_metrics = self.renderer.font_metrics()[font_spec.id].clone();
        self
    }

    /// Moves the writer to position `pos`.
    #[must_use]
    pub fn with_pos(mut self, pos: metrics::Point) -> Self {
        self.pos = pos;
        self
    }

    /// Re-aligns the writer to anchor `anchor`.
    #[must_use]
    pub fn align(mut self, anchor: metrics::anchor::X) -> Self {
        self.alignment = anchor;
        self
    }
}

/// We can use writers with Rust's formatting system.
impl<'r, R: super::Renderer + ?Sized> std::fmt::Write for Writer<'r, R> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let pos = self
            .pos
            .offset(-self.font_metrics.x_anchor_of_str(s, self.alignment), 0);
        self.pos = self
            .renderer
            .write(pos, self.font_spec, s)
            .map_err(|_| std::fmt::Error)?;

        Ok(())
    }

    /// Forces a formatting write to send one string to `write_str`.
    ///
    /// This is to make non-left-aligned writes work as one would expect.
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        let cow = args.as_str().map_or_else(
            || std::borrow::Cow::from(args.to_string()),
            std::borrow::Cow::from,
        );
        self.write_str(&cow)
    }
}
