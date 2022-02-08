/*! Widgets.

The (reference) UI for zombiesplit contains several self-rendering widgets,
each of which has access to the presenter state and a renderer.
*/

use super::{gfx, layout};

mod footer;
mod header;
pub mod label;
pub mod root;
mod split;
pub mod stack;
mod time;

pub use label::Label;
pub use root::Root;
pub use stack::Stack;

/// Trait for things that can render information from a presenter.
pub trait Widget<R: ?Sized>: super::layout::Layoutable {
    /// Type of state that this widget accepts.
    type State: ?Sized;

    /// Renders the widget.
    fn render(&self, r: &mut R, s: &Self::State) -> gfx::Result<()>;
}
