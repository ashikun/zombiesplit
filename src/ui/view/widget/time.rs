//! Structures common to time displaying widgets.

// TODO(@MattWindsor91): port footer to this

use super::{
    super::gfx::{
        colour, font,
        metrics::{conv::sat_i32, Anchor, Rect, Size},
        Renderer, Result, Writer,
    },
    IndexLayout,
};
use crate::model::time::position;
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

impl Layout {
    /// Calculates the minimal size required for this time widget.
    ///
    /// This should usually be used in conjunction with `layout`, using this size to produce the
    /// bounding box for this widget's layout.
    #[must_use]
    pub fn minimal_size(&self, ctx: super::LayoutContext) -> Size {
        let fm = &ctx.font_metrics[self.font_id];
        let raw: i32 = ctx
            .time_positions
            .iter()
            .map(|pos| position_width(fm, *pos) + fm.pad.w_i32())
            .sum();
        // fix off by one from above padding
        Size::from_i32s(raw - fm.pad.w_i32(), fm.span_h(1))
    }

    /// Recalculates the layout based on `ctx`.
    pub fn update(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;

        self.update_positions(ctx);
    }

    fn update_positions(&mut self, ctx: super::LayoutContext) {
        let fm = &ctx.font_metrics[self.font_id];

        let mut point = self.rect.top_left;

        self.positions.clear();
        self.positions.reserve(ctx.time_positions.len());

        for pos in ctx.time_positions {
            let size = Size::from_i32s(position_width(fm, *pos), fm.span_h(1));
            let rect = point.to_rect(size, Anchor::TOP_LEFT);
            self.positions.push(Position {
                index_layout: *pos,
                rect,
            });

            point.offset_mut(sat_i32(rect.size.w) + fm.pad.w_i32(), 0);
        }
    }

    /// Renders the time `time` onto `r` according to the layout's current state.
    ///
    /// This function is very generic in what sort of time `time` can be, in order to allow for
    /// both times and editors to be rendered using the same codepath.
    ///
    /// If `time` is `None`, a placeholder will be rendered instead.
    pub fn render<T: Index<position::Index, Output = W>, W: Display + ?Sized>(
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
                write!(w, "{}", &x[index_layout.index])?;
            } else {
                w.write_str(&"-".repeat(index_layout.num_digits.into()))?;
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
fn position_width(fm: &font::Metrics, pos: IndexLayout) -> i32 {
    let digits = fm.span_w(i32::from(pos.num_digits));
    let mut sigil = fm.span_w_str(unit_sigil(pos.index));
    // Making sure we only pad if there was a sigil
    if sigil != 0 {
        sigil += fm.pad.w_i32();
    }

    digits + sigil
}

/// The sigil displayed after the position indexed by `idx`.
const fn unit_sigil(idx: position::Index) -> &'static str {
    match idx {
        position::Index::Hours => ":",
        position::Index::Minutes => "'",
        position::Index::Seconds => "\"",
        position::Index::Milliseconds => "",
    }
}

/// Calculated positioning information for a time widget.
#[derive(Debug, Copy, Clone)]
struct Position {
    index_layout: IndexLayout,
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

impl Index<position::Index> for Colour {
    type Output = colour::Pair;

    fn index(&self, index: position::Index) -> &Self::Output {
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
    pub field: position::Index,
    /// The overriding colour.
    pub colour: colour::Pair,
}
