//! Mapping from SDL to presenter events.
use crate::{
    model::time::position,
    presenter::event::{Cursor, Event},
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
        // We don't allow entering hours yet, but this may change.
        Keycode::M => Some(Event::EnterField(position::Name::Minutes)),
        Keycode::S => Some(Event::EnterField(position::Name::Seconds)),
        Keycode::Period => Some(Event::EnterField(position::Name::Milliseconds)),
        Keycode::X => Some(Event::NewRun),
        Keycode::J | Keycode::Down | Keycode::Space => Some(Event::Cursor(Cursor::Down)),
        Keycode::K | Keycode::Up | Keycode::Backspace => Some(Event::Cursor(Cursor::Up)),
        Keycode::Escape => Some(Event::Quit),
        _ => None,
    }
}