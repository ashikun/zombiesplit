//! Label widgets.

use super::{
    super::{
        gfx::{
            colour, font,
            metrics::{anchor, Point, Rect, Size},
            render::Renderer,
            Result, Writer,
        },
        layout::{Context, Layoutable},
    },
    Widget,
};
use std::fmt::Write;

/// A widget that displays a static single-line string with a static font.
#[derive(Clone)]
pub struct Label {
    /// The most recently computed bounding box for the label.
    rect: Rect,

    /// The font spec for the label.
    pub font_spec: font::Spec,

    /// The minimum amount of expected characters in the label.
    pub min_chars: u8,

    /// The horizontal alignment of the label.
    pub align: anchor::X,
}

impl Label {
    /// Constructs a label with the given font specification.
    #[must_use]
    pub fn new(font_spec: font::Spec) -> Self {
        Self {
            rect: Rect::default(),
            font_spec,
            min_chars: 0,
            align: anchor::X::Left,
        }
    }

    /// Sets the alignment of the label.
    pub fn align(mut self, to: anchor::X) -> Self {
        self.align = to;
        self
    }

    /// Sets the minimum character amount of the label.
    pub fn min_chars(mut self, to: u8) -> Self {
        self.min_chars = to;
        self
    }

    /// Renders `str` onto the label with the given colour.
    ///
    /// This gives a finer degree of control than `render`.
    pub fn render_extended(
        &self,
        r: &mut impl Renderer,
        str: impl std::fmt::Display,
        colour: impl Into<Option<colour::fg::Id>>,
    ) -> Result<()> {
        let mut w = Writer::new(r)
            .with_pos(self.writer_pos())
            .with_font(self.override_font(colour))
            .align(self.align);
        Ok(write!(w, "{}", str)?)
    }

    fn writer_pos(&self) -> Point {
        self.rect.anchor(anchor::Anchor {
            x: self.align,
            y: anchor::Y::Top,
        })
    }

    fn override_font(&self, colour: impl Into<Option<colour::fg::Id>>) -> font::Spec {
        colour
            .into()
            .map_or(self.font_spec, |c| self.font_spec.id.coloured(c))
    }
}

impl Layoutable for Label {
    fn min_bounds(&self, parent_ctx: Context) -> Size {
        parent_ctx.font_metrics[self.font_spec.id].text_size(i32::from(self.min_chars), 1)
    }

    fn layout(&mut self, ctx: Context) {
        self.rect = ctx.bounds;
    }
}

impl<R: Renderer> Widget<R> for Label {
    type State = str;

    fn render(&self, r: &mut R, s: &Self::State) -> Result<()> {
        self.render_extended(r, s, None)
    }
}
