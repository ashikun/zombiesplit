//! Logic for drawing splits.

mod row;

use super::super::{
    super::presenter::state::State,
    gfx::{
        metrics::{Anchor, Length, Rect, Size},
        Renderer, Result,
    },
    layout::{self, Layoutable},
};

/// The split viewer widget.
#[derive(Default)]
pub struct Widget {
    /// The outer bounding box used for the widget.
    bounds: Rect,
    /// The inner, padded, bounding box used for the widget.
    rect: Rect,
    /// The split drawer set, containing enough drawers for one layout.
    rows: Vec<row::Row>,
}

impl Layoutable for Widget {
    fn min_bounds(&self, _parent_ctx: layout::Context) -> Size {
        // Splitsets fill in any of the space remaining after the header/footer/etc, so there is
        // no minimum bounds.
        Size::default()
    }

    fn actual_bounds(&self) -> Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        let ctx = ctx.padded();
        self.rect = ctx.bounds;
        self.rows = rows(ctx);
    }
}

impl<R: Renderer> super::Widget<R> for Widget {
    type State = State;

    fn render(&self, r: &mut R, s: &Self::State) -> Result<()> {
        let iter = SplitIndexIter::new(self.rows.len(), s.num_splits(), s.cursor_position());

        for (i, row) in iter.zip(self.rows.iter()) {
            if let Some(split) = s.split_at_index(i) {
                row.render(r, split)?;
            }
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

fn row_bounds(ctx: layout::Context, split_h: Length, ix: Length) -> Rect {
    Rect {
        top_left: ctx.bounds.point(0, ix * split_h, Anchor::TOP_LEFT),
        size: Size {
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
        // Find out where we want to put our cursor.
        // Ideally, we want to have the cursor be halfway through the slots.
        // We floor to avoid the possibility of a 1-slot scroll not showing the cursor.
        let ideal_cursor_slot = self.num_slots / 2;

        // This means that our first approximation of the first split is that many slots
        // above the cursor.  A saturating subtraction means that, if the cursor is on the
        // first few slots, it will progressively move up to the top of the splits.
        let split = self.cursor.saturating_sub(ideal_cursor_slot);

        // Nudge the first split down to make sure that we fill all of the available slots.
        let gap = (split + self.num_slots).saturating_sub(self.num_splits);
        split.saturating_sub(gap)
    }

    fn need_windowing(&self) -> bool {
        self.cur_split == 0 && self.num_slots < self.num_splits
    }
}
