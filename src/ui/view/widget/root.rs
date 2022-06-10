//! The root widget.
//!
//! The root widget is a vertical stack of a header, split set, and footer.

use ugly::metrics;

use super::{
    super::{
        super::presenter::State,
        config::layout::WidgetSet,
        gfx,
        layout::{self, Layoutable},
        update::{self, Updatable},
    },
    header, split, Footer, Status, Widget,
};

/// The root widget.
pub struct Root(super::Stack<Component>);

impl Layoutable for Root {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        self.0.min_bounds(parent_ctx)
    }

    fn actual_bounds(&self) -> metrics::Size {
        self.0.actual_bounds()
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.0.layout(ctx);
    }
}

impl Updatable for Root {
    type State = State;

    fn update(&mut self, ctx: &update::Context, s: &Self::State) {
        self.0.update(ctx, s);
    }
}

impl<'r, R: gfx::Renderer<'r>> Widget<R> for Root {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        self.0.render(r)
    }
}

impl Root {
    /// Constructs a new root widget using the given layout configuration.
    #[must_use]
    pub fn new(cfg: &WidgetSet) -> Self {
        let mut stack = super::Stack::new(metrics::Axis::Vertical);
        stack.push(Component::Header(header::Widget::default()), 0);
        stack.push(Component::Splitset(split::Widget::default()), 1);
        stack.push(Component::Footer(Footer::new(&cfg.footer)), 0);
        stack.push(Component::Status(Status::default()), 0);
        Self(stack)
    }
}

/// Enumeration of the various widgets stored on the root.
///
/// This mainly serves to multiplex the widgets into a homogeneous stack without needing a box.
enum Component {
    /// Represents the header widget.
    Header(header::Widget),
    /// Represents the splitset widget.
    Splitset(split::Widget),
    /// Represents the footer widget.
    Footer(Footer),
    /// Represents the status widget.
    Status(Status),
}

impl Layoutable for Component {
    fn min_bounds(&self, parent_ctx: layout::Context) -> metrics::Size {
        match self {
            Self::Header(h) => h.min_bounds(parent_ctx),
            Self::Splitset(s) => s.min_bounds(parent_ctx),
            Self::Footer(f) => f.min_bounds(parent_ctx),
            Self::Status(t) => t.min_bounds(parent_ctx),
        }
    }

    fn actual_bounds(&self) -> metrics::Size {
        match self {
            Self::Header(h) => h.actual_bounds(),
            Self::Splitset(s) => s.actual_bounds(),
            Self::Footer(f) => f.actual_bounds(),
            Self::Status(t) => t.actual_bounds(),
        }
    }

    fn layout(&mut self, ctx: layout::Context) {
        match self {
            Self::Header(h) => h.layout(ctx),
            Self::Splitset(s) => s.layout(ctx),
            Self::Footer(f) => f.layout(ctx),
            Self::Status(t) => t.layout(ctx),
        }
    }
}

impl Updatable for Component {
    type State = State;

    fn update(&mut self, ctx: &update::Context, state: &Self::State) {
        match self {
            Self::Header(h) => h.update(ctx, state),
            Self::Splitset(s) => s.update(ctx, state),
            Self::Footer(f) => f.update(ctx, &state.footer),
            Self::Status(t) => t.update(ctx, state),
        };
    }
}

impl<'r, R: gfx::Renderer<'r>> Widget<R> for Component {
    fn render(&self, r: &mut R) -> ugly::Result<()> {
        match self {
            Self::Header(h) => h.render(r),
            Self::Splitset(s) => s.render(r),
            Self::Footer(f) => f.render(r),
            Self::Status(t) => t.render(r),
        }
    }
}
