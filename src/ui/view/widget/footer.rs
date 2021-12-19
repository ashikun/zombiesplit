//! The split total widget.

use std::borrow::Cow;
use std::fmt::Write;

use super::{
    super::{
        super::presenter::state::{self, footer},
        gfx::{
            self, colour, font,
            metrics::{self, conv::u32_or_zero, Anchor, Size},
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
            let h = ctx.font_metrics[row.time.font_id].span_h(1);
            let row_rect = top_left.to_rect(
                Size {
                    w,
                    h: u32_or_zero(h),
                },
                Anchor::TOP_LEFT,
            );
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
            Row::new(RowType::Comparison, font::Id::Large),
            Row::new(RowType::UpToCursor, font::Id::Medium),
        ]);
    }
}

/// Sub-widget for a row in the footer.
struct Row {
    /// The type of row being shown in this.
    row_type: footer::RowType,

    /// The top-left position of the label.
    label_top_left: metrics::Point,
    /// The layout information for the time.
    time: super::time::Layout,
}

impl Row {
    /// Constructs a row with the given type and time font.
    fn new(row_type: footer::RowType, time_font: font::Id) -> Self {
        let mut time = super::time::Layout::default();
        time.font_id = time_font;
        Self {
            row_type,
            label_top_left: metrics::Point::default(),
            time,
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
        time: &Option<Cow<pace::PacedTime>>,
    ) -> gfx::Result<()> {
        let pace = time.as_ref().map_or_else(pace::Pace::default, |t| t.pace);

        let t = time.as_ref().map(|x| &x.as_ref().time);
        self.time.render(r, t, &colour::fg::Id::Pace(pace).into())?;

        Ok(())
    }
}

impl Widget<state::State> for Row {
    fn layout(&mut self, ctx: LayoutContext) {
        self.label_top_left = ctx.bounds.top_left;

        let time_rect = ctx
            .bounds
            .point(0, 0, Anchor::TOP_RIGHT)
            .to_rect(self.time.minimal_size(ctx), Anchor::TOP_RIGHT);
        self.time.update(ctx.with_bounds(time_rect));
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::State) -> gfx::Result<()> {
        self.render_label(r)?;
        self.render_time(r, &s.footer.get(self.row_type))?;
        Ok(())
    }
}
