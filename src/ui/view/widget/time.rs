//! Structures common to time displaying widgets.

// TODO(@MattWindsor91): port footer to this

use super::{
    super::gfx::{
        colour, font,
        metrics::{
            conv::{sat_i32, u32_or_zero},
            Rect, Size,
        },
        Renderer, Result, Writer,
    },
    IndexLayout, LayoutContext,
};
use crate::model::time::position;
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::Index,
};

/// Layout information for a general time widget.
#[derive(Debug, Default)]
pub struct Layout {
    /// Bounding box of the layout.
    rect: Rect,

    /// Font used for the time display.
    font_id: font::Id,

    /// Rects of each position in the time widget.
    position_rect_map: HashMap<position::Index, Rect>,
}

impl Layout {
    /// Recalculates the layout based on `ctx`.
    pub fn update(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;

        self.update_positions(ctx)
    }

    fn update_positions(&mut self, ctx: super::LayoutContext) {
        let fm = &ctx.font_metrics[self.font_id];
        let mut point = self.rect.top_left;

        for pos in ctx.time_positions {
            let w = u32_or_zero(position_width(fm, pos));
            let rect = point.to_rect(Size {
                w,
                h: u32_or_zero(fm.span_h(1)),
            });

            self.position_rect_map.insert(pos.index, rect);
            point.offset_mut(sat_i32(rect.size.w), 0);
        }
    }

    /// Renders the time `time` onto `r` according to the layout's current state.
    ///
    /// This function is very generic in what sort of time `time` can be, in order to allow for
    /// both times and editors to be rendered using the same codepath.
    pub fn render<T: Index<position::Index, Output = W>, W: Display + ?Sized>(
        &self,
        r: &mut dyn Renderer,
        time: &T,
        colour: Colour,
    ) -> Result<()> {
        try_fill(r, self.rect, &colour)?;

        for (index, rect) in &self.position_rect_map {
            let col = &colour[*index];
            try_fill(r, *rect, &colour)?;

            // TODO(@MattWindsor91): create w outside the loop
            let mut w = Writer::new(r).with_font_id(self.font_id);
            w = w.with_pos(rect.top_left).with_colour(col.fg);

            write!(w, "{}{}", &time[*index], unit_sigil(*index))?;
        }
        Ok(())
    }
}

fn try_fill(r: &mut dyn Renderer, rect: Rect, colour: &Colour) -> Result<()> {
    if let Some(bg) = colour.base.bg {
        r.set_bg_colour(bg);
        r.fill(rect.grow(1))?;
    }
    Ok(())
}

/// Pre-calculates the size of a time widget.
#[must_use]
pub fn size(ctx: LayoutContext, font: font::Id) -> Size {
    let fm = &ctx.font_metrics[font];
    let raw: i32 = ctx
        .time_positions
        .iter()
        .map(|pos| position_width(fm, pos) + sat_i32(fm.pad.w))
        .sum();
    // fix off by one from above padding
    Size {
        w: u32_or_zero(raw).saturating_sub(fm.pad.w),
        h: u32_or_zero(fm.span_h(1)),
    }
}

fn position_width(fm: &font::Metrics, pos: &IndexLayout) -> i32 {
    // Need to remember the padding between the digits and the sigil.
    fm.span_w(pos.num_digits as i32) + sat_i32(fm.pad.w) + fm.span_w_str(unit_sigil(pos.index))
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
            .map(|x| &x.colour)
            .unwrap_or(&self.base)
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
