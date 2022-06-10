//! Label widgets.

use ugly::metrics;

use super::{
    super::{
        gfx::{colour, font, Renderer, Writer},
        layout::{self, Layoutable},
        update::{self, Updatable},
    },
    Widget,
};

/// A widget that displays a static single-line string with a static font.
#[derive(Clone)]
pub struct Label {
    /// The most recently computed bounding box for the label.
    bounds: metrics::Rect,

    /// The writer for the label.
    writer: Writer,

    /// The minimum amount of expected characters in the label.
    pub min_chars: u8,
}

impl Label {
    /// Constructs a label with the given font specification.
    #[must_use]
    pub fn new(font_spec: font::Spec) -> Self {
        let mut writer = Writer::new();
        writer.set_font_spec(font_spec);

        Self {
            bounds: metrics::Rect::default(),
            writer,
            min_chars: 0,
        }
    }

    /// Sets the alignment of the label.
    pub fn align(mut self, to: metrics::anchor::X) -> Self {
        self.writer.align_to(to);
        self
    }

    /// Sets the minimum character amount of the label.
    pub fn min_chars(mut self, to: u8) -> Self {
        self.min_chars = to;
        self
    }

    /// Renders `str` onto the label with the given colour.
    ///
    /// This gives a finer degree of control than `update`.
    pub fn update_extended(
        &mut self,
        ctx: &update::Context,
        str: impl std::fmt::Display,
        colour: impl Into<Option<colour::fg::Id>>,
    ) {
        self.writer.move_to(self.writer_pos());
        if let Some(c) = colour.into() {
            self.writer.set_fg(c);
        }

        self.writer.set_string(&str);
        self.writer.layout(ctx.font_metrics);
    }

    fn writer_pos(&self) -> metrics::Point {
        self.bounds.anchor(metrics::anchor::Anchor {
            x: self.writer.alignment(),
            y: metrics::anchor::Y::Top,
        })
    }
}

impl Layoutable for Label {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        parent_ctx
            .font_metrics(self.writer.font_spec().id)
            .text_size(i32::from(self.min_chars), 1)
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;
    }
}

impl Updatable for Label {
    type State = str;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        self.update_extended(ctx, s, None);
    }
}

impl<'r, R: Renderer<'r>> Widget<R> for Label {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.writer.render(r)
    }
}
