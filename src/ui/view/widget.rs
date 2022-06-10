//! Widgets.
//!
//! The (reference) UI for zombiesplit contains several self-rendering widgets,
//! each of which has access to the presenter state and a renderer.  These widgets are semi-retained
//! inasmuch as they are redrawn each cycle, but maintain internal state updated potentially less
//! frequently, and layouts are calculated only if the window resizes.

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
pub trait Widget<R: ?Sized>: super::layout::Layoutable + super::update::Updatable {
    /// Renders the widget onto `r`.
    ///
    /// This will be called every cycle, and should reflect the results of the last `update` call.
    fn render(&self, r: &mut R) -> ugly::Result<()>;
}
