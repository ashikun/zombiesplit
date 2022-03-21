//! The status bar widget and its implementations.
use ugly::metrics;

use super::{
    super::{
        super::presenter,
        gfx::{colour, font, Renderer},
    },
    layout::{self, Layoutable},
    Label, Widget,
};

/// The status bar.
///
/// The status bar consists of multiple labels, each projecting information about the current
/// status of the user interface.
pub struct Status {
    /// Outer bounding box of the status bar.
    bounds: metrics::Rect,

    /// Padded inner bounding box of the status bar.
    inner_bounds: metrics::Rect,

    /// Label for the current mode of the user interface.
    mode: Label,

    /// Label for the current split position.
    split_position: Label,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            bounds: metrics::Rect::default(),
            inner_bounds: metrics::Rect::default(),

            mode: Label::new(STATUS_SPEC),
            split_position: Label::new(STATUS_SPEC)
                .min_chars(3)
                .align(metrics::anchor::X::Right),
        }
    }
}

const STATUS_SPEC: font::Spec = font::Id::Small.coloured(colour::fg::Id::Status);

impl Layoutable for Status {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        parent_ctx.pad_size(metrics::Size::stack_horizontally(
            self.mode.min_bounds(parent_ctx),
            self.split_position.min_bounds(parent_ctx),
        ))
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        let ctx = ctx.padded();
        self.inner_bounds = ctx.bounds;

        // The two labels overlap (for now).
        self.mode.layout(ctx);
        self.split_position.layout(ctx);
    }
}

impl<R: Renderer> Widget<R> for Status {
    type State = presenter::State;

    fn render(&self, r: &mut R, s: &Self::State) -> ugly::Result<()> {
        r.fill(self.bounds, colour::bg::Id::Status)?;

        self.mode.render_extended(r, &s.mode, None)?;

        self.render_position(r, s)
    }
}

impl Status {
    fn render_position(&self, r: &mut impl Renderer, s: &presenter::State) -> ugly::Result<()> {
        let position = format! {"{}/{}", s.cursor_position() + 1, s.num_splits()};
        self.split_position.render(r, &position)
    }
}
