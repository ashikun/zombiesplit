/*! View events.

These are analogous to presenter events, but tell the view that certain things have happened to
the window that require handling at the view level. */

/// A view event.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    /// The window has resized to the given dimensions.
    Resize(ugly::metrics::Size),
}
