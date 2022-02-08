/*! The root widget.

The root widget is a vertical stack of a header, split set, and footer.
*/

use super::{
    super::{
        super::presenter::State,
        config::layout::WidgetSet,
        gfx,
        layout::{self, Layoutable},
    },
    footer, header, split, Widget,
};

/// The root widget.
pub struct Root(super::Stack<Component>);

impl layout::Layoutable for Root {
    fn min_bounds(&self, parent_ctx: layout::Context) -> gfx::metrics::Size {
        self.0.min_bounds(parent_ctx)
    }

    fn actual_bounds(&self) -> gfx::metrics::Size {
        self.0.actual_bounds()
    }

    fn layout(&mut self, ctx: layout::Context) {
        self.0.layout(ctx);
    }
}

impl<R: gfx::Renderer> Widget<R> for Root {
    type State = State;

    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()> {
        self.0.render(r, s)
    }
}

impl Root {
    /// Constructs a new root widget using the given layout configuration.
    #[must_use]
    pub fn new(cfg: &WidgetSet) -> Self {
        let mut stack = super::Stack::new(gfx::metrics::Axis::Vertical);
        stack.push(Component::Header(header::Widget::default()), 0);
        stack.push(Component::Splitset(split::Widget::default()), 1);
        stack.push(Component::Footer(footer::Footer::new(&cfg.footer)), 0);
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
    /// Represents the status widget.
    Footer(footer::Footer),
}

impl Layoutable for Component {
    fn min_bounds(&self, parent_ctx: layout::Context) -> gfx::metrics::Size {
        match self {
            Self::Header(h) => h.min_bounds(parent_ctx),
            Self::Splitset(s) => s.min_bounds(parent_ctx),
            Self::Footer(f) => f.min_bounds(parent_ctx),
        }
    }

    fn actual_bounds(&self) -> gfx::metrics::Size {
        match self {
            Self::Header(h) => h.actual_bounds(),
            Self::Splitset(s) => s.actual_bounds(),
            Self::Footer(f) => f.actual_bounds(),
        }
    }

    fn layout(&mut self, ctx: layout::Context) {
        match self {
            Self::Header(h) => h.layout(ctx),
            Self::Splitset(s) => s.layout(ctx),
            Self::Footer(f) => f.layout(ctx),
        }
    }
}

impl<R: gfx::Renderer> Widget<R> for Component {
    type State = State;

    fn render(&self, r: &mut R, state: &Self::State) -> gfx::Result<()> {
        match self {
            Self::Header(h) => h.render(r, state),
            Self::Splitset(s) => s.render(r, state),
            Self::Footer(f) => f.render(r, &state.footer),
        }
    }
}
