//! Header display.

use std::fmt::Write;

use super::super::{
    super::presenter::State,
    gfx::{self, colour, font, metrics, Renderer, Writer},
    layout,
};
use crate::model::game::category::{AttemptInfo, Info};

/// Views information about the run in the form of a header.
#[derive(Default)]
pub struct Widget {
    /// The bounding box for the header widget.
    rect: metrics::Rect,

    /// The position of the category name.
    category_pos: metrics::Point,

    /// The position of the top-right of the attempts counter.
    attempts_pos: metrics::Point,
}

impl layout::Layoutable for Widget {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        // TODO(@MattWindsor91): this is clearly a stack, and there should be a less manual way
        // of dealing with it.
        metrics::Size::stack_vertically(
            header_bounds(parent_ctx),
            metrics::Size::stack_horizontally(
                // TODO(@MattWindsor91): attempts spec != category spec
                category_bounds(parent_ctx),
                category_bounds(parent_ctx),
            ),
        )
        .grow(2 * parent_ctx.config.window.padding)
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.rect = ctx.padded().bounds;

        let header_metrics = &ctx.font_metrics[HEADER_FONT_SPEC.id];
        let one_below_header = header_metrics.span_h(1);

        self.category_pos = self
            .rect
            .point(0, one_below_header, metrics::Anchor::TOP_LEFT);

        self.attempts_pos = self
            .rect
            .point(0, one_below_header, metrics::Anchor::TOP_RIGHT);
    }
}

fn header_bounds(ctx: layout::Context) -> metrics::Size {
    text_bounds(ctx, HEADER_FONT_SPEC.id)
}

fn category_bounds(ctx: layout::Context) -> metrics::Size {
    text_bounds(ctx, CATEGORY_FONT_SPEC.id)
}

fn text_bounds(ctx: layout::Context, font_id: font::Id) -> metrics::Size {
    // We don't yet require space to be laid out for a particular number of chars.
    ctx.font_metrics[font_id].text_size(0, 1)
}

impl<R: Renderer> super::Widget<R> for Widget {
    type State = State;

    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()> {
        self.render_meta(r, &s.game_category)?;
        self.render_attempt(r, &s.attempt)?;
        Ok(())
    }
}

const HEADER_FONT_SPEC: font::Spec = font::Spec {
    id: font::Id::Large,
    colour: colour::fg::Id::Header,
};

const CATEGORY_FONT_SPEC: font::Spec = font::Spec {
    id: font::Id::Medium,
    colour: colour::fg::Id::Header,
};

impl Widget {
    fn render_meta(&self, r: &mut dyn Renderer, meta: &Info) -> gfx::Result<()> {
        self.write_header(r, meta)?;
        self.write_category(r, meta)?;
        Ok(())
    }

    fn write_header(&self, r: &mut dyn Renderer, meta: &Info) -> gfx::Result<()> {
        Writer::new(r)
            .with_font(HEADER_FONT_SPEC)
            .with_pos(self.rect.top_left)
            .write_str(&meta.game)?;
        Ok(())
    }

    fn write_category(&self, r: &mut dyn Renderer, meta: &Info) -> gfx::Result<()> {
        Writer::new(r)
            .with_font(CATEGORY_FONT_SPEC)
            .with_pos(self.category_pos)
            .write_str(&meta.category)?;
        Ok(())
    }

    fn render_attempt(&self, r: &mut dyn Renderer, attempt: &AttemptInfo) -> gfx::Result<()> {
        let mut w = Writer::new(r)
            .with_font(CATEGORY_FONT_SPEC)
            .with_pos(self.attempts_pos)
            .align(metrics::anchor::X::Right);
        write!(w, "#{} ({})", attempt.total, attempt.completed)?;
        Ok(())
    }
}
