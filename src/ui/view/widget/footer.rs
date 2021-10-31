//! The split total widget.

use std::borrow::Cow;
use std::fmt::Write;

use super::{
    super::{
        super::presenter::state::{self, footer},
        gfx::{
            self, colour, font,
            metrics::{
                self,
                anchor::{self, Anchor},
                conv::u32_or_zero,
                Size,
            },
            Renderer, Writer,
        },
    },
    LayoutContext, Widget,
};
use crate::model::comparison::pace;
use crate::ui::presenter::state::footer::RowType;

/// The footer widget.
#[derive(Default)]
pub struct Footer {
    /// The bounding box for the footer widget.
    rect: metrics::Rect,
    /// The rows configured into this Footer.
    rows: Vec<Row>,
}

impl super::Widget<state::State> for Footer {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;

        if self.rows.is_empty() {
            self.init_rows();
        }

        let w = self.rect.size.w;
        let mut top_left = self.rect.top_left;
        for row in &mut self.rows {
            let h = ctx.font_metrics[row.time_font].span_h(1);
            let row_rect = top_left.to_rect(Size {
                w,
                h: u32_or_zero(h),
            });
            row.layout(ctx.with_bounds(row_rect));
            top_left.offset_mut(0, h);
        }
    }

    fn children(&self) -> Vec<&dyn Widget<state::State>> {
        self.rows
            .iter()
            .map(|x| -> &dyn Widget<state::State> { x })
            .collect()
    }
}

impl Footer {
    fn init_rows(&mut self) {
        // TODO(@MattWindsor91): make this configurable.
        self.rows.extend([
            Row::new(RowType::Total, font::Id::Large),
            Row::new(RowType::Comparison, font::Id::Medium),
            Row::new(RowType::UpToCursor, font::Id::Medium),
        ]);
    }
}

/// Sub-widget for a row in the footer.
struct Row {
    /// The type of row being shown in this.
    row_type: footer::RowType,
    /// The font to use for the time itself.
    time_font: font::Id,

    /// The top-left position of the label.
    label_top_left: metrics::Point,
    /// The top-right position of the time.
    time_top_right: metrics::Point,
}

impl Row {
    /// Constructs a row with the given type and time font.
    fn new(row_type: footer::RowType, time_font: font::Id) -> Self {
        Self {
            row_type,
            time_font,
            label_top_left: metrics::Point::default(),
            time_top_right: metrics::Point::default(),
        }
    }

    fn render_label(&self, r: &mut dyn Renderer) -> gfx::Result<()> {
        let mut w = Writer::new(r).with_pos(self.label_top_left);
        write!(w, "{}", self.row_type)?;
        Ok(())
    }

    fn render_time(
        &self,
        r: &mut dyn Renderer,
        time: Option<Cow<pace::PacedTime>>,
    ) -> gfx::Result<()> {
        // TODO(@MattWindsor91): harmonise this with the split setup.
        let pace = time.as_ref().map_or_else(pace::Pace::default, |t| t.pace);
        let fg = colour::fg::Id::Pace(pace);

        let mut w = Writer::new(r)
            .with_pos(self.time_top_right)
            .align(anchor::X::Right)
            .with_font(font::Spec {
                id: self.time_font,
                colour: fg,
            });
        if let Some(t) = time {
            write!(w, "{}'{}\"{}", t.time.mins, t.time.secs, t.time.millis)?;
        } else {
            w.write_str("--'--\"---")?;
        }
        Ok(())
    }
}

impl Widget<state::State> for Row {
    fn layout(&mut self, ctx: LayoutContext) {
        self.label_top_left = ctx.bounds.top_left;
        self.time_top_right = ctx.bounds.point(0, 0, Anchor::TOP_RIGHT);
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::State) -> gfx::Result<()> {
        self.render_label(r)?;
        self.render_time(r, s.footer.get(self.row_type))?;
        Ok(())
    }
}
