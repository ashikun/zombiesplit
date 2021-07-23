//! Widget trait and associated code.

use super::{
    super::{super::Presenter, error::Result},
    render::Renderer,
};

/// Trait for things that can render information from a presenter.
pub trait Widget {
    /// Renders information from `p` onto the renderer `r`.
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()>;
}
