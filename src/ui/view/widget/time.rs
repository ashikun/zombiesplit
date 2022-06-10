//! Structures common to time displaying widgets.

use std::{fmt::Display, ops::Index};
use ugly::{
    font::Metrics,
    metrics::{self, Point, Rect},
};

use super::super::{
    super::super::model::timing::time::{self, format::Component},
    gfx::{colour, font, Renderer, Writer},
    layout::{self, Layoutable},
    update,
};

/// Layout information for a general time widget.
#[derive(Debug, Default, Clone)]
pub struct Layout {
    /// Font to use for this time widget.
    pub font_id: font::Id,

    /// Bounding box of the layout.
    bounds: Rect,

    /// Layouts of each position in the time widget.
    positions: Vec<Position>,

    /// Most recently assigned colours for this time.
    colour: Colour,
}

impl Layoutable for Layout {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        let fm = parent_ctx.font_metrics(self.font_id);
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
        let fm = ctx.font_metrics(self.font_id);

        let mut point = self.bounds.top_left;

        self.positions.clear();

        for component in ctx.config.time.components() {
            let (rect, position) = self.make_position(fm, point, component);
            self.positions.push(position);

            point.offset_mut(rect.size.w + fm.pad.w, 0);
        }
    }

    fn make_position(
        &mut self,
        fm: &Metrics,
        point: Point,
        component: &Component,
    ) -> (Rect, Position) {
        let size = metrics::Size {
            w: position_width(fm, *component),
            h: fm.span_h(1),
        };

        let mut writer = Writer::new();
        writer.set_font(self.font_id);
        writer.move_to(point);

        let rect = point.to_rect(size, metrics::Anchor::TOP_LEFT);
        let position = Position {
            component: *component,
            rect,
            writer,
        };
        (rect, position)
    }

    /// Updates the layout with the time `time`.
    ///
    /// This function is very generic in what sort of time `time` can be, in order to allow for
    /// both times and editors to be rendered using the same codepath.
    ///
    /// If `time` is `None`, a placeholder will be rendered instead.
    pub fn update<T: Index<time::Position, Output = W>, W: Display + ?Sized>(
        &mut self,
        ctx: &update::Context,
        time: Option<&T>,
        colour: Colour,
    ) {
        self.colour = colour;

        for pos in &mut self.positions {
            match pos.component {
                Component::Position { position, width } => {
                    let col = colour[position];
                    pos.writer.set_fg(col.fg);
                    pos.writer.set_string(
                        &(if let Some(x) = time {
                            format!("{:width$}", &x[position], width = width)
                        } else {
                            "-".repeat(width)
                        }),
                    );
                }
                Component::Delimiter(c) => {
                    pos.writer.set_fg(colour.base.fg);
                    pos.writer.set_string(&c);
                }
            }
            pos.writer.layout(ctx.font_metrics);
        }
    }

    /// Renders the time `time` onto `r` according to the layout's current state.
    ///
    /// This function is very generic in what sort of time `time` can be, in order to allow for
    /// both times and editors to be rendered using the same codepath.
    ///
    /// If `time` is `None`, a placeholder will be rendered instead.
    pub fn render<'r>(&self, r: &mut impl Renderer<'r>) -> ugly::Result<()> {
        try_fill(r, self.bounds, self.colour.base)?;

        self.positions.iter().try_for_each(|pos| {
            if let Component::Position { position, .. } = pos.component {
                try_fill(r, pos.rect, self.colour[position])?;
            }
            pos.writer.render(r)
        })
    }
}

fn try_fill<'r>(r: &mut impl Renderer<'r>, rect: Rect, colour: colour::Pair) -> ugly::Result<()> {
    r.fill(rect.grow(1), colour.bg)
}

/// Calculates the width of a position in a time widget, excluding any padding.
fn position_width(metrics: &Metrics, c: Component) -> metrics::Length {
    match c {
        Component::Position { width, .. } => metrics.span_w(width.try_into().unwrap_or_default()),
        Component::Delimiter(c) => metrics.span_w_char(c),
    }
}

/// Calculated positioning information for a time widget.
#[derive(Debug, Clone)]
struct Position {
    /// Writer used for this position.
    writer: Writer,
    /// The component that has been positioned.
    component: Component,
    /// The bounding box created for the component.
    rect: Rect,
}

/// Time colouring information.
#[derive(Copy, Clone, Debug, Default)]
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
#[derive(Copy, Clone, Debug)]
pub struct FieldColour {
    /// The field that triggers this override.
    pub field: time::Position,
    /// The overriding colour.
    pub colour: colour::Pair,
}
