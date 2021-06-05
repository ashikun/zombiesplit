//! Mapping from SDL to presenter events.
use crate::{
    model::time::position,
    presenter::{
        cursor,
        event::{Edit, Event},
    },
};

/// Maps an event from SDL into [Event].
pub fn from_sdl(e: &sdl2::event::Event) -> Option<Event> {
    match e {
        sdl2::event::Event::Quit { .. } => Some(Event::Quit),
        sdl2::event::Event::KeyDown {
            keycode: Some(k), ..
        } => from_key(*k),
        _ => None,
    }
}

fn from_key(k: sdl2::keyboard::Keycode) -> Option<Event> {
    use sdl2::keyboard::Keycode;
    match k {
        // Editing
        Keycode::Num0 => Some(Event::digit(0)),
        Keycode::Num1 => Some(Event::digit(1)),
        Keycode::Num2 => Some(Event::digit(2)),
        Keycode::Num3 => Some(Event::digit(3)),
        Keycode::Num4 => Some(Event::digit(4)),
        Keycode::Num5 => Some(Event::digit(5)),
        Keycode::Num6 => Some(Event::digit(6)),
        Keycode::Num7 => Some(Event::digit(7)),
        Keycode::Num8 => Some(Event::digit(8)),
        Keycode::Num9 => Some(Event::digit(9)),
        Keycode::Backspace => Some(Event::Edit(Edit::Remove)),
        // Time fields
        // We don't allow entering hours yet, but this may change.
        Keycode::M => Some(Event::EnterField(position::Name::Minutes)),
        Keycode::S => Some(Event::EnterField(position::Name::Seconds)),
        Keycode::Period => Some(Event::EnterField(position::Name::Milliseconds)),
        // Cursor motions
        Keycode::J | Keycode::Down | Keycode::Space | Keycode::Return => Some(Event::Cursor(cursor::Motion::Down)),
        Keycode::K | Keycode::Up => Some(Event::Cursor(cursor::Motion::Up)),
        // Top-level commands
        Keycode::H | Keycode::Left => Some(Event::Undo),
        Keycode::L | Keycode::Right => Some(Event::Commit),
        Keycode::X | Keycode::Delete => Some(Event::Delete),
        Keycode::Z => Some(Event::NewRun),
        Keycode::Escape => Some(Event::Quit),
        _ => None,
    }
}
