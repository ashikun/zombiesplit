//! Events understood by the user interface (mapped onto SDL events).
use crate::model::time::position;

/// High-level event, translated from a SDL event.
pub enum Event {
    /// Start entering a field at a particular position.
    EnterField(position::Name),
    /// Start a new run.
    NewRun,
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
            // We don't allow entering hours yet, but this may change.
            Keycode::M => Some(Self::EnterField(position::Name::Minutes)),
            Keycode::S => Some(Self::EnterField(position::Name::Seconds)),
            Keycode::Period => Some(Self::EnterField(position::Name::Milliseconds)),
            Keycode::X => Some(Self::NewRun),
            Keycode::J | Keycode::Down | Keycode::Space => Some(Self::CursorDown),
            Keycode::K | Keycode::Up | Keycode::Backspace => Some(Self::CursorUp),
            sdl2::keyboard::Keycode::Escape => Some(Self::Quit),
            _ => None,
        }
    }
}
