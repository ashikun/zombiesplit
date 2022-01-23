/// Footer rows.
use super::super::{
    super::{
        super::presenter::state::{self, footer},
        config,
        gfx::{self, colour, metrics, metrics::Anchor, Renderer, Writer},
        layout,
    },
    time::Layout,
    Widget,
};
use crate::model::timing::comparison;
use std::{borrow::Cow, fmt::Write};

/// Sub-widget for a row in the footer.
pub struct Row {
    /// The type of row being shown in this.
    pub(super) row_type: footer::RowType,

    /// The top-left position of the label.
    pub(super) label_top_left: metrics::Point,
    /// The layout information for the time.
    pub(super) time: Layout,
}

impl Row {
    /// Constructs a row with the given initial layout information and no set positioning.
    pub(super) fn new(layout: &config::layout::Row) -> Self {
        let mut time = Layout::default();
        time.font_id = layout.font;
        Self {
            row_type: layout.contents,
            label_top_left: metrics::Point::default(),
            time,
        }
    }

    fn render_label(&self, r: &mut impl Renderer) -> gfx::Result<()> {
        let mut w = Writer::new(r).with_pos(self.label_top_left);
        write!(w, "{}", self.row_type)?;
        Ok(())
    }

    fn render_time(
        &self,
        r: &mut impl Renderer,
        time: &Option<Cow<comparison::PacedTime>>,
    ) -> gfx::Result<()> {
        let pace = time
            .as_ref()
            .map_or_else(comparison::Pace::default, |t| t.pace);

        let t = time.as_ref().map(|x| &x.as_ref().time);
        self.time.render(r, t, &colour::fg::Id::Pace(pace).into())?;

        Ok(())
    }
}

impl layout::Layoutable for Row {
    fn layout(&mut self, ctx: layout::Context) {
        self.label_top_left = ctx.bounds.top_left;

        let time_rect = ctx
            .bounds
            .point(0, 0, Anchor::TOP_RIGHT)
            .to_rect(self.time.minimal_size(ctx), Anchor::TOP_RIGHT);
        self.time.layout(ctx.with_bounds(time_rect));
    }
}

impl<R: Renderer> Widget<R> for Row {
    type State = state::Footer;

    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()> {
        self.render_label(r)?;
        self.render_time(r, &s.get(self.row_type))?;
        Ok(())
    }
}
