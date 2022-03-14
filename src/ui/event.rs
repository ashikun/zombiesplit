/*! Top layer of interface events.

User interface events take the form of either:

- presenter events, which forward to the presenter to be consumed or sent to
  the server/model (eg: 'move cursor up');
- view events, which affect the view (eg: 'size changed', 'renderer tick').

We assume that both events come from a `Pump`, which is the effective top level
driver for the user interface. */

use super::presenter;

/// The event pump.
pub trait Pump {
    /// Pumps all current events available into a vector.
    fn pump(&mut self) -> Vec<Event>;
}

/// A view or presenter event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// A view event.
    View(super::view::Event),
    /// A presenter event.
    Presenter(presenter::Event),
}

impl Event {
    /// Shorthand for producing a digit.
    #[must_use]
    pub const fn digit(digit: u8) -> Self {
        Self::Presenter(presenter::event::Event::digit(digit))
    }

    /// Shorthand for constructing a modal event.
    #[must_use]
    pub const fn modal(m: presenter::mode::Event) -> Event {
        Event::Presenter(presenter::event::Event::Modal(m))
    }

    /// Shorthand for constructing a cursor motion event.
    #[must_use]
    pub const fn motion(m: presenter::state::cursor::Motion) -> Event {
        Event::Presenter(presenter::event::Event::motion(m))
    }
}
