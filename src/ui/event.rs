/*! Top layer of interface events.

User interface events take the form of either:

- presenter events, which forward to the presenter to be consumed or sent to
  the server/model (eg: 'move cursor up');
- view events, which affect the view (eg: 'size changed', 'renderer tick').

We assume that both events come from a `Pump`, which is the effective top level
driver for the user interface. */

/// The event pump.
pub trait Pump {
    /// Pumps all current events available into an interator.
    fn pump(&mut self) -> Vec<Event>;
}

/// A view or presenter event.
pub enum Event {
    /// A view event.
    View(View),
    /// A presenter event.
    Presenter(super::presenter::event::Event),
}

impl Event {
    /// Shorthand for producing a digit.
    #[must_use]
    pub fn digit(digit: u8) -> Self {
        Self::Presenter(super::presenter::event::Event::digit(digit))
    }

    /// Shorthand for constructing a modal event.
    #[must_use]
    pub fn modal(m: super::presenter::event::Modal) -> Event {
        Event::Presenter(super::presenter::event::Event::Modal(m))
    }
}

/// A view event.
pub enum View {
    /// The window has resized to the given dimensions.
    Resize(super::view::gfx::metrics::Size),
}
