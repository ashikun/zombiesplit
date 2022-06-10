//! Logic for drawing splits.

mod row;

use ugly::metrics;

use super::super::{
    super::presenter::state::State,
    gfx::Renderer,
    layout::{self, Layoutable},
    update::{self, Updatable},
};

/// The split viewer widget.
#[derive(Default)]
pub struct Widget {
    /// The outer bounding box used for the widget.
    bounds: metrics::Rect,
    /// The inner, padded, bounding box used for the widget.
    rect: metrics::Rect,
    /// The split drawer set, containing enough drawers for one layout.
    rows: Vec<row::Row>,
}

impl Layoutable for Widget {
    fn min_bounds(&self, _parent_ctx: layout::Context) -> metrics::Size {
        // Splitsets fill in any of the space remaining after the header/footer/etc, so there is
        // no minimum bounds.
        metrics::Size::default()
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        let ctx = ctx.padded();
        self.rect = ctx.bounds;
        self.rows = rows(ctx);
    }
}

impl Updatable for Widget {
    type State = State;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        let iter = SplitIndexIter::new(self.rows.len(), s.num_splits(), s.cursor_position());

        for (i, row) in iter.zip(self.rows.iter_mut()) {
            if let Some(split) = s.split_at_index(i) {
                row.update(ctx, split);
            }
        }
    }
}

impl<'r, R: Renderer<'r>> super::Widget<R> for Widget {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        for row in &self.rows {
            row.render(r)?;
        }
        Ok(())
    }
}

/// Constructs a vector of row widgets according to `ctx`.
fn rows(ctx: layout::Context) -> Vec<row::Row> {
    // Create a prototype row, measure it to see how tall it is, then use that to work out how
    // many we can fit in this layout.

    // TODO(@MattWindsor91): padding

    let row = row::Row::default();
    let split_h = row.min_bounds(ctx).h;

    let n_splits = ctx.bounds.size.h / split_h;
    (0..n_splits)
        .map(|n| {
            let mut r = row.clone();
            r.layout(ctx.with_bounds(row_bounds(ctx, split_h, n)));
            r
        })
        .collect()
}

fn row_bounds(
    ctx: layout::Context,
    split_h: metrics::Length,
    ix: metrics::Length,
) -> metrics::Rect {
    metrics::Rect {
        top_left: ctx.bounds.point(0, ix * split_h, metrics::Anchor::TOP_LEFT),
        size: metrics::Size {
            w: ctx.bounds.size.w,
            h: split_h,
        },
    }
}

/// Iterator producing the raw split indices to fit into split rows.
struct SplitIndexIter {
    num_slots: usize,
    num_splits: usize,
    cur_split: usize,
    cursor: usize,
}

impl Iterator for SplitIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Have we run out of splits?
        if self.num_splits <= self.cur_split {
            None
        } else {
            if self.need_windowing() {
                self.cur_split = self.first_windowed_split();
            }

            let index = self.cur_split;
            self.cur_split += 1;
            Some(index)
        }
    }
}

impl SplitIndexIter {
    #[must_use]
    pub fn new(num_slots: usize, num_splits: usize, cursor: usize) -> Self {
        // We can't provide more slots than we have splits!
        let num_slots = num_slots.min(num_splits);
        Self {
            num_slots,
            num_splits,
            cursor,
            cur_split: 0,
        }
    }

    fn first_windowed_split(&self) -> usize {
        // Ideally, we want to have the cursor be halfway through the slots.
        // We floor to avoid the possibility of a 1-slot scroll not showing the cursor.
        let ideal_cursor_slot = self.num_slots / 2;

        // Our first approximation of the first split is that `ideal_cursor_slot` slots above the
        // cursor.  A saturating subtraction means that, if the cursor is on the first few slots,
        // it will progressively move up to the top without scrolling.
        let split = self.cursor.saturating_sub(ideal_cursor_slot);

        // Nudge the first split down to make sure that we fill all of the available slots.
        let gap = (split + self.num_slots).saturating_sub(self.num_splits);
        split.saturating_sub(gap)
    }

    fn need_windowing(&self) -> bool {
        self.cur_split == 0 && self.num_slots < self.num_splits
    }
}
