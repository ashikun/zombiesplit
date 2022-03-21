//! Widgets.
//!
//! The (reference) UI for zombiesplit contains several self-rendering widgets,
//! each of which has access to the presenter state and a renderer.

use super::layout;

mod footer;
mod header;
mod label;
mod root;
mod split;
mod stack;
mod status;
mod time;

pub use footer::Footer;
pub use label::Label;
pub use root::Root;
pub use stack::Stack;
pub use status::Status;

/// Trait for things that can render information from a presenter.
pub trait Widget<R: ?Sized>: super::layout::Layoutable {
    /// Type of state that this widget accepts.
    type State: ?Sized;

    /// Renders the widget onto `r`.
    fn render(&self, r: &mut R, s: &Self::State) -> ugly::Result<()>;
}
