//! The [Updatable] trait and its associated [Context].
//!
//! This module represents the updating phase of the widget lifecycle.  This phase passes the
//! current state to the widget and requests that it prepare its model of what it intends to send
//! for rendering.

use super::gfx::font::Map;

/// Trait for things that can be updated using a state.
pub trait Updatable {
    /// Type of external state that this widget accepts.
    type State: ?Sized;

    /// Updates the widget's internal state according to the current external state.
    ///
    /// This will be called every cycle, before rendering.  This may change later.
    fn update(&mut self, ctx: &Context, s: &Self::State);
}

/// Context used when performing updates.
pub struct Context<'m> {
    /// A source of font metrics.
    ///
    /// This can be used for working out how large a piece of text might be.
    pub font_metrics: &'m Map<ugly::font::Metrics>,
}
