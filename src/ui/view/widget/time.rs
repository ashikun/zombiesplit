//! Structures common to time displaying widgets.

use super::{
    super::gfx::{
        colour, font,
        metrics::{Anchor, Rect, Size},
        Renderer, Result, Writer,
    },
    layout,
};
use crate::model::time;
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
            .positions()
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

        for pos in ctx.config.time.positions() {
            let size = Size {
                w: position_width(fm, *pos),
                h: fm.span_h(1),
            };
            let rect = point.to_rect(size, Anchor::TOP_LEFT);
            self.positions.push(Position {
                index_layout: *pos,
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
        r: &mut dyn Renderer,
        time: Option<&T>,
        colour: &Colour,
    ) -> Result<()> {
        try_fill(r, self.rect, colour)?;

        for Position { index_layout, rect } in &self.positions {
            let col = &colour[index_layout.index];
            try_fill(r, *rect, colour)?;

            // TODO(@MattWindsor91): create w outside the loop
            let mut w = Writer::new(r).with_font_id(self.font_id);
            w = w.with_pos(rect.top_left).with_colour(col.fg);

            if let Some(x) = time {
                write!(
                    w,
                    "{:width$}",
                    &x[index_layout.index],
                    width = index_layout.num_digits
                )?;
            } else {
                w.write_str(&"-".repeat(index_layout.num_digits))?;
            }

            w.write_str(unit_sigil(index_layout.index))?;
        }
        Ok(())
    }
}

fn try_fill(r: &mut dyn Renderer, rect: Rect, colour: &Colour) -> Result<()> {
    if let Some(bg) = colour.base.bg {
        r.fill(rect.grow(1), bg)?;
    }
    Ok(())
}

/// Calculates the width of a position in a time widget, excluding any padding.
fn position_width(fm: &font::Metrics, pos: time::format::Position) -> i32 {
    let nd: i32 = pos.num_digits.try_into().unwrap_or_default();
    let digits = fm.span_w(nd);
    let mut sigil = fm.span_w_str(unit_sigil(pos.index));
    // Making sure we only pad if there was a sigil
    if sigil != 0 {
        sigil += fm.pad.w;
    }

    digits + sigil
}

/// The sigil displayed after the position indexed by `idx`.
const fn unit_sigil(idx: time::Position) -> &'static str {
    // TODO(@MattWindsor91): consider making these user configurable.
    match idx {
        time::Position::Hours => ":",
        time::Position::Minutes => "'",
        time::Position::Seconds => "\"",
        time::Position::Milliseconds => "",
    }
}

/// Calculated positioning information for a time widget.
#[derive(Debug, Copy, Clone)]
struct Position {
    index_layout: time::format::Position,
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
