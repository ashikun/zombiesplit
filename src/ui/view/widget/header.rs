//! Header display.
use ugly::metrics;

use super::{
    super::{
        super::presenter::State,
        gfx::{colour, font, Renderer},
        layout::{self, Layoutable},
        update::{self, Updatable},
    },
    label::Label,
};

/// Views information about the run in the form of a header.
pub struct Widget {
    /// The outer bounding box for the header widget.
    bounds: metrics::Rect,

    /// The effective (padded) bounding box for the header widget.
    rect: metrics::Rect,

    /// The game label.
    name: Label,

    /// The category label.
    category: Label,

    /// The attempts counter label.
    attempts: Label,
}

impl Default for Widget {
    fn default() -> Self {
        Self {
            bounds: metrics::Rect::default(),
            rect: metrics::Rect::default(),
            name: Label::new(HEADER_FONT_SPEC),
            category: Label::new(CATEGORY_FONT_SPEC),
            // TODO(@MattWindsor91): attempts spec != category spec
            attempts: Label::new(CATEGORY_FONT_SPEC)
                .min_chars(3)
                .align(metrics::anchor::X::Right),
        }
    }
}

impl Layoutable for Widget {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        // TODO(@MattWindsor91): this is clearly a stack, and there should be a less manual way
        // of dealing with it.
        metrics::Size::stack_vertically(
            self.name.min_bounds(parent_ctx),
            metrics::Size::stack_horizontally(
                self.category.min_bounds(parent_ctx),
                self.attempts.min_bounds(parent_ctx),
            ),
        )
        .grow(2 * parent_ctx.config.window.padding)
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;
        self.rect = ctx.padded().bounds;

        // TODO(@MattWindsor91): common pattern, abstract.
        let mut name_rect = self.rect;
        name_rect.size.h = self.name.min_bounds(ctx).h;
        self.name.layout(ctx.with_bounds(name_rect));

        let mut category_rect = self.rect;
        category_rect.top_left.y += name_rect.size.h;
        category_rect.size.h = self.category.min_bounds(ctx).h;
        self.category.layout(ctx.with_bounds(category_rect));

        // TODO(@MattWindsor91): don't overlap these rects
        self.attempts.layout(ctx.with_bounds(category_rect));
    }
}

impl Updatable for Widget {
    type State = State;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        self.name.update(ctx, &s.game_category.game);
        self.category.update(ctx, &s.game_category.category);
        self.attempts.update_extended(ctx, &s.attempt, None);
    }
}

impl<'r, R: Renderer<'r>> super::Widget<R> for Widget {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.name.render(r)?;
        self.category.render(r)?;
        self.attempts.render(r)?;
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
