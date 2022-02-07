//! Label widgets.

use super::{
    super::{
        gfx::{
            colour, font,
            metrics::{anchor, Rect, Size},
            render::Renderer,
            Result, Writer,
        },
        layout::{Context, Layoutable},
    },
    Widget,
};
use crate::ui::view::gfx::colour::fg::Id;
use crate::ui::view::gfx::font::Spec;
use crate::ui::view::gfx::metrics::Point;
use std::fmt::Write;

/// A widget that displays a static single-line string with a static font.
#[derive(Clone)]
pub struct Label {
    /// The most recently computed bounding box for the label.
    rect: Rect,

    /// The font spec for the label.
    font_spec: font::Spec,

    /// The minimum amount of expected characters in the label.
    min_chars: u8,

    /// The horizontal alignment of the label.
    align: anchor::X,
}

impl Label {
    /// Constructs a new label widget with the given `font_spec`.
    #[must_use]
    pub fn new(font_spec: font::Spec, min_chars: u8, align: anchor::X) -> Self {
        Self {
            rect: Rect::default(),
            font_spec,
            min_chars,
            align,
        }
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

    fn override_font(&self, colour: impl Into<Option<Id>>) -> Spec {
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
