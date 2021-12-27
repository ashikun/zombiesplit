//! Structures common to time displaying widgets.

use super::{
    super::gfx::{
        colour, font,
        metrics::{Anchor, Length, Rect, Size},
        Renderer, Result, Writer,
    },
    layout,
};
use crate::model::time::{self, format::Component};
use std::{
    fmt::{Display, Write},
    ops::Index,
};

/// Layout information for a general time widget.
#[derive(Debug, Default)]
pub struct Layout {
    /// Bounding box of the layout.
    rect: Rect,

    /// Font used for the time display.
    pub font_id: font::Id,

    /// Layouts of each position in the time widget.
    positions: Vec<Position>,
}

impl layout::Layoutable for Layout {
    fn layout(&mut self, ctx: layout::Context) {
        self.rect = ctx.bounds;

        self.update_positions(ctx);
    }
}

impl Layout {
    /// Calculates the minimal size required for this time widget.
    ///
    /// This should usually be used in conjunction with `layout`, using this size to produce the
    /// bounding box for this widget's layout.
    #[must_use]
    pub fn minimal_size(&self, ctx: layout::Context) -> Size {
        let fm = &ctx.font_metrics[self.font_id];
        let raw: i32 = ctx
            .config
            .time
            .components()
            .map(|pos| position_width(fm, *pos) + fm.pad.w)
            .sum();
        // fix off by one from above padding
        Size {
            w: raw - fm.pad.w,
            h: fm.span_h(1),
        }
    }

    fn update_positions(&mut self, ctx: layout::Context) {
        let fm = &ctx.font_metrics[self.font_id];

        let mut point = self.rect.top_left;

        self.positions.clear();

        for component in ctx.config.time.components() {
            let size = Size {
                w: position_width(fm, *component),
                h: fm.span_h(1),
            };
            let rect = point.to_rect(size, Anchor::TOP_LEFT);
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
    ) -> Result<()> {
        let mut w = Writer::new(r).with_font_id(self.font_id);

        try_fill(&mut w, self.rect, colour.base)?;

        for pos in &self.positions {
            w = w.with_pos(pos.rect.top_left);
            match pos.component {
                Component::Position { index, num_digits } => {
                    let col = colour[index];
                    try_fill(&mut w, pos.rect, col)?;
                    w = w.with_colour(col.fg);

                    if let Some(x) = time {
                        write!(w, "{:width$}", &x[index], width = num_digits)?;
                    } else {
                        w.write_str(&"-".repeat(num_digits))?;
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

fn try_fill(r: &mut impl Renderer, rect: Rect, colour: colour::Pair) -> Result<()> {
    if let Some(bg) = colour.bg {
        r.fill(rect.grow(1), bg)?;
    }
    Ok(())
}

/// Calculates the width of a position in a time widget, excluding any padding.
fn position_width(metrics: &font::Metrics, c: Component) -> Length {
    match c {
        Component::Position { num_digits, .. } => {
            metrics.span_w(num_digits.try_into().unwrap_or_default())
        }
        Component::Delimiter(c) => metrics.span_w_char(c),
    }
}

/// Calculated positioning information for a time widget.
#[derive(Debug, Copy, Clone)]
struct Position {
    /// The component that has been positioned.
    component: time::format::Component,
    /// The bounding box created for the component.
    rect: Rect,
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
