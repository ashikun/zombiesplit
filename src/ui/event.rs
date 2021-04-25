//! Events understood by the user interface (mapped onto SDL events).

/// High-level event, translated from a SDL event.
pub enum Event {
    /// Move the cursor up.
    CursorUp,
    /// Move the cursor down.
    CursorDown,
    /// Quit the program.
    Quit,
}

impl Event {
    /// Maps an event from SDL into [Event].
    pub fn from_sdl(e: &sdl2::event::Event) -> Option<Self> {
        use sdl2::event::Event;
        match e {
            Event::Quit { .. } => Some(Self::Quit),
            Event::KeyDown {
                keycode: Some(k), ..
            } => Self::from_key(*k),
            _ => None,
        }
    }

    fn from_key(k: sdl2::keyboard::Keycode) -> Option<Self> {
        use sdl2::keyboard::Keycode;
        match k {
            Keycode::J | Keycode::Down => Some(Self::CursorDown),
            Keycode::K | Keycode::Up => Some(Self::CursorUp),
            sdl2::keyboard::Keycode::Escape => Some(Self::Quit),
            _ => None,
        }
    }
}
