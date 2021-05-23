//! Events understood by the user interface (mapped onto SDL events).
use crate::model::time::position;

/// High-level event, translated from a SDL event.
pub enum Event {
    /// Start editing a field at a particular position.
    EnterField(position::Name),
    /// Perform an event on the currently open editor.
    Edit(Edit),
    /// Start a new run.
    NewRun,
    /// Move the cursor.
    Cursor(Cursor),
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
            Keycode::Num0 => Some(Self::digit(0)),
            Keycode::Num1 => Some(Self::digit(1)),
            Keycode::Num2 => Some(Self::digit(2)),
            Keycode::Num3 => Some(Self::digit(3)),
            Keycode::Num4 => Some(Self::digit(4)),
            Keycode::Num5 => Some(Self::digit(5)),
            Keycode::Num6 => Some(Self::digit(6)),
            Keycode::Num7 => Some(Self::digit(7)),
            Keycode::Num8 => Some(Self::digit(8)),
            Keycode::Num9 => Some(Self::digit(9)),
            // We don't allow entering hours yet, but this may change.
            Keycode::M => Some(Self::EnterField(position::Name::Minutes)),
            Keycode::S => Some(Self::EnterField(position::Name::Seconds)),
            Keycode::Period => Some(Self::EnterField(position::Name::Milliseconds)),
            Keycode::X => Some(Self::NewRun),
            Keycode::J | Keycode::Down | Keycode::Space => Some(Self::Cursor(Cursor::Down)),
            Keycode::K | Keycode::Up | Keycode::Backspace => Some(Self::Cursor(Cursor::Up)),
            Keycode::Escape => Some(Self::Quit),
            _ => None,
        }
    }

    /// Shorthand for producing a field event.
    fn digit(digit: u8) -> Self {
        Self::Edit(Edit::Add(digit))
    }
}

/// An edit event.
pub enum Edit {
    /// Add the given digit to the current editor.
    Add(u8),
    /// Remove the last digit from the current editor.
    Remove,
}

/// A cursor movement event.
#[derive(Copy, Clone)]
pub enum Cursor {
    /// Move the cursor up.
    Up,
    /// Move the cursor down.
    Down,
}
