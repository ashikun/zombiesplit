//! Stacking widgets.

use crate::ui::view::update::{Context, Updatable};
use ugly::metrics;

use super::{
    super::layout::{self, Layoutable},
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
    contents: Vec<Entry<W>>,
}

/// We can layout a stack by laying out its individual components, with some flexing.
impl<W: Layoutable> Layoutable for Stack<W> {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        // We can't use compute_min_bounds here, because &self is immutable.
        self.orientation.stack_many(
            self.contents
                .iter()
                .map(|x| x.widget.min_bounds(parent_ctx)),
        )
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.bounds.size
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.bounds = ctx.bounds;

        self.compute_min_bounds(ctx);

        let length_per_ratio = self.gap().checked_div(self.ratio_sum()).unwrap_or_default();

        // Only proceed with the rest of the layout if there is at least one item.
        // The last item gets handled differently, see below.
        if let Some((last, except_last)) = self.contents.split_last_mut() {
            let mut axis = self.bounds.size[self.orientation];
            let perp_axis = self.bounds.size[self.orientation.normal()];

            let mut top_left = self.bounds.top_left;
            for entry in except_last {
                // 'axis' comes into this calculation because we might have run out of allocation
                // midway through the stack, even though we have some non-flexible elements left.
                let allocation = entry
                    .allocation(length_per_ratio, self.orientation)
                    .clamp(0, axis);

                let size = self.orientation.size(allocation, perp_axis);
                entry.layout(ctx.with_bounds(metrics::Rect { top_left, size }));

                axis -= allocation;
                assert!(0 <= axis, "axis should never become negative");
                top_left[self.orientation] += allocation;
            }

            // Fill the rest of the stack with the remaining allocation.
            let size = self.orientation.size(axis.max(0), perp_axis);
            last.layout(ctx.with_bounds(metrics::Rect { top_left, size }));
        }
    }
}

/// Stacks are updatable, distributing updates to their children.
///
/// Each child widget must have the same state.
impl<S, W: Updatable<State = S>> Updatable for Stack<W> {
    type State = S;

    fn update(&mut self, ctx: &Context, s: &Self::State) {
        for c in &mut self.contents {
            c.update(ctx, s);
        }
    }
}

/// Stacks are widgets, distributing rendering to their children.
///
/// Each child widget must have the same rendering state.
impl<R, S, W: Widget<R, State = S>> Widget<R> for Stack<W> {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.contents.iter().try_for_each(|c| c.render(r))
    }
}

impl<W: Layoutable> Stack<W> {
    /// Pre-computes the minimum bounds for each component in this stack.
    fn compute_min_bounds(&mut self, ctx: layout::Context) {
        for entry in &mut self.contents {
            entry.min_bounds = entry.min_bounds(ctx);
        }
    }

    fn gap(&self) -> metrics::Length {
        let result = self.bounds.size[self.orientation] - self.occupied_size()[self.orientation];
        // The amount to fill might be negative if the minimum sizes of the elements can't be
        // satisfied, in which case we clamp back to 0 and instead just clip at the bottom.
        result.max(0)
    }

    /// Gets the total stacked size of all components in this stack that are not flexible.
    ///
    /// Expects `compute_min_bounds` to have been called.
    fn occupied_size(&self) -> metrics::Size {
        self.orientation.stack_many(
            self.contents
                .iter()
                .filter_map(|x| (x.ratio == 0).then(|| x.min_bounds)),
        )
    }
}

impl<W> Stack<W> {
    fn ratio_sum(&self) -> metrics::Length {
        self.contents
            .iter()
            .map(|x| metrics::Length::from(x.ratio))
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

    /// Pushes a widget and ratio onto the end of the stack.
    pub fn push(&mut self, widget: W, ratio: u8) {
        self.contents.push(Entry::new(widget, ratio));
    }

    /// Extends the stack with the given iterable of widget/ratio pairs.
    pub fn extend(&mut self, widgets: impl IntoIterator<Item = (W, u8)>) {
        self.contents.extend(
            widgets
                .into_iter()
                .map(|(widget, ratio)| Entry::new(widget, ratio)),
        );
    }
}

struct Entry<W> {
    /// The widget.
    widget: W,
    /// The widget's most recently computed bounding box.
    min_bounds: metrics::Size,
    /// The widget's ratio.
    ratio: u8,
    /// Whether the widget is visible.
    visible: bool,
}

impl<W> Entry<W> {
    fn new(widget: W, ratio: u8) -> Self {
        Self {
            widget,
            ratio,
            min_bounds: metrics::Size::default(),
            visible: false,
        }
    }
}

impl<W: Layoutable> Entry<W> {
    fn allocation(&self, gap_per_ratio: i32, axis: metrics::Axis) -> metrics::Length {
        if self.ratio == 0 {
            self.min_bounds[axis]
        } else {
            metrics::Length::from(self.ratio) * gap_per_ratio
        }
    }
}

/// Delegates to the actual bounds of the entry.
impl<W: Layoutable> Layoutable for Entry<W> {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        self.widget.min_bounds(parent_ctx)
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.widget.actual_bounds()
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.widget.layout(ctx);
        self.visible = !self.actual_bounds().is_zero();
    }
}

/// Entries are updatable, distributing updates to their embedded widget.
impl<S, W: Updatable<State = S>> Updatable for Entry<W> {
    type State = S;

    fn update(&mut self, ctx: &Context, s: &Self::State) {
        if self.visible {
            self.widget.update(ctx, s);
        }
    }
}

/// Entries are widgets, distributing rendering to their embedded widget.
impl<R, S, W: Widget<R, State = S>> Widget<R> for Entry<W> {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        if self.visible {
            self.widget.render(r)?;
        }
        Ok(())
    }
}
