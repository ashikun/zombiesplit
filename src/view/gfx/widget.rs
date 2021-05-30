//! Widget trait and associated code.

use super::render::Renderer;
use crate::{presenter::Presenter, view::error::Result};

/// Trait for things that can render information from a presenter.
pub trait Widget {
    /// Renders information from `p` onto the renderer `r`.
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()>;
}
