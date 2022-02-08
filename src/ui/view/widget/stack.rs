//! Stacking widgets.

use super::{
    super::gfx::metrics,
    layout::{self, Layoutable},
    Widget,
};

/// Homogeneous stack of widgets.
///
/// The stack is similar to a very stripped-back flexbox, in that each item has a particular
/// ratio that, when nonzero, causes it to acquire a particular share of the remaining
pub struct Stack<W> {
    /// The bounding box of the stack.
    bounds: metrics::Rect,

    /// The orientation of the stack.
    orientation: metrics::Axis,
    /// The contents of the stack, with their ratios.
    contents: Vec<(W, u8)>,
}

/// We can layout a stack by laying out its individual components, with some flexing.
impl<W: Layoutable> Layoutable for Stack<W> {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        self.orientation
            .stack_many(self.contents.iter().map(|(w, _)| w.min_bounds(parent_ctx)))
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        let sum = self.ratio_sum();
        let mut gap = self.gap(self.occupied_size(ctx));
        assert!(0 <= gap, "gap should not be negative");

        // Only proceed with the rest of the layout if there is at least one item.
        // The last item gets handled differently, see below.
        if let Some(((last, _), except_last)) = self.contents.split_last_mut() {
            let gap_per_ratio = gap.checked_div(sum).unwrap_or_default();

            let perp_axis = self.bounds.size[self.orientation.normal()];

            let mut top_left = self.bounds.top_left;
            for (w, r) in except_last {
                let allocation = if *r == 0 {
                    w.min_bounds(ctx)[self.orientation]
                } else {
                    metrics::Length::from(*r) * gap_per_ratio
                };

                let size = self.orientation.size(allocation, perp_axis);
                w.layout(ctx.with_bounds(metrics::Rect { top_left, size }));

                gap -= allocation;
                top_left[self.orientation] += allocation;
            }

            // Fill the rest of the stack with the remaining allocation.
            let size = self.orientation.size(gap.min(0), perp_axis);
            last.layout(ctx.with_bounds(metrics::Rect { top_left, size }));
        }
    }
}

/// Stacks are widgets, distributing rendering to their children.
///
/// Each child widget must have the same rendering state.
impl<R, S, W: Widget<R, State = S>> Widget<R> for Stack<W> {
    type State = S;

    fn render(&self, r: &mut R, s: &Self::State) -> crate::ui::view::gfx::Result<()> {
        for (w, _) in &self.contents {
            w.render(r, s)?;
        }
        Ok(())
    }
}

impl<W: Layoutable> Stack<W> {
    /// Gets the total stacked size of all components in this stack that are not flexible.
    fn occupied_size(&self, ctx: layout::Context) -> metrics::Size {
        self.orientation.stack_many(
            self.contents
                .iter()
                .filter_map(|(w, r)| (*r == 0).then(|| w.min_bounds(ctx))),
        )
    }
}

impl<W> Stack<W> {
    fn gap(&self, occupied: metrics::Size) -> metrics::Length {
        let result = self.bounds.size[self.orientation] - occupied[self.orientation];
        // The amount to fill might be negative if the minimum sizes of the elements can't be
        // satisfied, in which case we clamp back to 0 and instead just clip at the bottom.
        result.max(0)
    }

    fn ratio_sum(&self) -> metrics::Length {
        self.contents
            .iter()
            .map(|(_, r)| metrics::Length::from(*r))
            .sum()
    }

    /// Constructs a stack of widgets with the given orientation.
    #[must_use]
    pub fn new(orientation: metrics::Axis) -> Self {
        Self {
            bounds: metrics::Rect::default(),
            orientation,
            contents: vec![],
        }
    }

    /// Extends the stack with the given iterable of widget/ratio pairs.
    pub fn extend(&mut self, widgets: impl IntoIterator<Item = (W, u8)>) {
        self.contents.extend(widgets);
    }
}
