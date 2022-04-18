//! Structures common to time displaying widgets.

use std::{
    fmt::{Display, Write},
    ops::Index,
};
use ugly::metrics;

use super::{
    super::{
        super::super::model::timing::time::{self, format::Component},
        gfx::{colour, font, Renderer},
    },
    layout,
};

/// Layout information for a general time widget.
#[derive(Debug, Default, Clone)]
pub struct Layout {
    /// Bounding box of the layout.
    bounds: metrics::Rect,

    /// Font used for the time display.
    pub font_id: font::Id,

    /// Layouts of each position in the time widget.
    positions: Vec<Position>,
}

impl layout::Layoutable for Layout {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        let fm = &parent_ctx.font_metrics(self.font_id);
        let raw: i32 = parent_ctx
            .config
            .time
            .components()
            .map(|pos| position_width(fm, *pos) + fm.pad.w)
            .sum();
        // fix off by one from above padding
        metrics::Size {
            w: raw - fm.pad.w,
            h: fm.span_h(1),
        }
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        self.update_positions(ctx);
    }
}

impl Layout {
    fn update_positions(&mut self, ctx: layout::Context) {
        let fm = &ctx.font_metrics(self.font_id);

        let mut point = self.bounds.top_left;

        self.positions.clear();

        for component in ctx.config.time.components() {
            let size = metrics::Size {
                w: position_width(fm, *component),
                h: fm.span_h(1),
            };
            let rect = point.to_rect(size, metrics::Anchor::TOP_LEFT);
            self.positions.push(Position {
                component: *component,
                rect,
            });

            point.offset_mut(rect.size.w + fm.pad.w, 0);
        }
    }

    /// Renders the time `time` onto `r` according to the layout's current state.
    ///
    /// This function is very generic in what sort of time `time` can be, in order to allow for
    /// both times and editors to be rendered using the same codepath.
    ///
    /// If `time` is `None`, a placeholder will be rendered instead.
    pub fn render<T: Index<time::Position, Output = W>, W: Display + ?Sized>(
        &self,
        r: &mut impl Renderer,
        time: Option<&T>,
        colour: &Colour,
    ) -> ugly::Result<()> {
        let mut w = ugly::text::Writer::new(r).with_font_id(self.font_id);

        try_fill(&mut w, self.bounds, colour.base)?;

        for pos in &self.positions {
            w = w.with_pos(pos.rect.top_left);
            match pos.component {
                Component::Position { position, width } => {
                    let col = colour[position];
                    try_fill(&mut w, pos.rect, col)?;
                    w = w.with_colour(col.fg);

                    if let Some(x) = time {
                        write!(w, "{:width$}", &x[position], width = width)?;
                    } else {
                        w.write_str(&"-".repeat(width))?;
                    }
                }
                Component::Delimiter(c) => {
                    w = w.with_colour(colour.base.fg);
                    w.write_char(c)?;
                }
            }
        }

        Ok(())
    }
}

fn try_fill(r: &mut impl Renderer, rect: metrics::Rect, colour: colour::Pair) -> ugly::Result<()> {
    r.fill(rect.grow(1), colour.bg)
}

/// Calculates the width of a position in a time widget, excluding any padding.
fn position_width(metrics: &ugly::font::Metrics, c: Component) -> metrics::Length {
    match c {
        Component::Position { width, .. } => metrics.span_w(width.try_into().unwrap_or_default()),
        Component::Delimiter(c) => metrics.span_w_char(c),
    }
}

/// Calculated positioning information for a time widget.
#[derive(Debug, Copy, Clone)]
struct Position {
    /// The component that has been positioned.
    component: time::format::Component,
    /// The bounding box created for the component.
    rect: metrics::Rect,
}

/// Time colouring information.
pub struct Colour {
    /// The base colour.
    pub base: colour::Pair,
    /// Any field override.
    pub field: Option<FieldColour>,
}

/// Colours a time using a flat foreground colour.
impl From<colour::fg::Id> for Colour {
    fn from(fg: colour::fg::Id) -> Self {
        Colour {
            base: fg.into(),
            field: None,
        }
    }
}

impl Index<time::Position> for Colour {
    type Output = colour::Pair;

    fn index(&self, index: time::Position) -> &Self::Output {
        self.field
            .as_ref()
            .filter(|x| x.field == index)
            .map_or(&self.base, |x| &x.colour)
    }
}

/// Field override colouring information.
///
/// This is generally used for field editors.
pub struct FieldColour {
    /// The field that triggers this override.
    pub field: time::Position,
    /// The overriding colour.
    pub colour: colour::Pair,
}
