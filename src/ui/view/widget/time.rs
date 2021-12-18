//! Structures common to time displaying widgets.

// TODO(@MattWindsor91): port footer to this

use super::{
    super::gfx::{
        colour, font,
        metrics::{Point, Rect},
        Renderer, Result, Writer,
    },
    IndexLayout,
};
use crate::model::{comparison::PacedTime, time::position};
use crate::ui::view::gfx::metrics;
use crate::ui::view::gfx::metrics::conv::sat_i32;
use std::fmt::Display;
use std::ops::Index;
use std::{collections::HashMap, fmt::Write};

/// Layout information for a general time widget.
#[derive(Debug, Default)]
pub struct Layout {
    // TODO(@MattWindsor91): make `rect` not public
    /// Bounding box of the layout.
    pub rect: Rect,

    /// Font used for the time display.
    pub font_id: font::Id,

    /// Top-left of each position in the time widget.
    position_top_left_map: HashMap<position::Index, Point>,
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
            self.position_top_left_map.insert(pos.index, point);
            point.offset_mut(position_width(fm, pos), 0);
        }
    }

    /// Renders the time `time` onto `r` according to the layout's current state.
    ///
    /// This function is very generic in what sort of time `time` can be, in order to allow for
    /// both times and editors to be rendered using the same codepath.
    pub fn render<T: Index<position::Index, Output = W>, W: Display + ?Sized>(
        &self,
        r: &mut dyn Renderer,
        time: T,
        colour: colour::fg::Id,
    ) -> Result<()> {
        let mut w = Writer::new(r).with_font(self.font_id.coloured(colour));
        for (index, top_left) in &self.position_top_left_map {
            w = w.with_pos(*top_left);
            write!(w, "{}{}", &time[*index], unit_sigil(*index))?;
        }
        Ok(())
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

/// Calculates the size of an editor rectangle, given a text sizer.
#[must_use]
pub fn size(t: &font::Metrics) -> metrics::Size {
    metrics::Size::from_i32s(t.span_w_str(PLACEHOLDER), t.span_h(1))
}

/// Template rendered underneath the editor.
pub const PLACEHOLDER: &str = "  '  \"   ";
