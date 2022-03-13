//! Mapping between SDL events and presenter events.

use super::super::{
    super::model::{session, timing::time},
    event::{self, Event},
    presenter::{
        self,
        mode::{self, event::Edit},
        state::cursor,
    },
    view,
};
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

/// An SDL2 keymap.
pub struct Keymap(HashMap<Keycode, Event>);

/// Wrapper over SDL event pumps to promote them into `event::Pump` instances.
pub struct Pump {
    /// The event pump to poll for events.
    pub pump: sdl2::EventPump,
    /// The keymap to use when deciphering key events.
    pub keymap: Keymap,
}

impl Pump {
    /// Constructs a pump with the default keymap.
    #[must_use]
    pub fn new(pump: sdl2::EventPump) -> Self {
        Self {
            pump,
            keymap: Keymap::default(),
        }
    }
}

impl event::Pump for Pump {
    fn pump(&mut self) -> Vec<Event> {
        self.pump
            .poll_iter()
            .filter_map(|x| from_sdl(&x, &self.keymap))
            .collect()
    }
}

/// Maps an event from SDL into [Event].
#[must_use]
fn from_sdl(e: &sdl2::event::Event, keymap: &Keymap) -> Option<Event> {
    match e {
        sdl2::event::Event::Window {
            win_event: sdl2::event::WindowEvent::SizeChanged(w, h),
            ..
        } => Some(Event::View(view::Event::Resize(view::gfx::metrics::Size {
            w: *w,
            h: *h,
        }))),
        sdl2::event::Event::Quit { .. } => Some(Event::Presenter(presenter::Event::Quit)),
        sdl2::event::Event::KeyDown {
            keycode: Some(k), ..
        } => keymap.get(*k),
        _ => None,
    }
}

impl Default for Keymap {
    fn default() -> Self {
        let mut result = Keymap(HashMap::new());
        result.extend(DIGITS.iter().copied().map(|(k, d)| (k, Event::digit(d))));
        result.extend(
            POSITION_KEYS
                .iter()
                .copied()
                .map(|(k, p)| (k, Event::modal(mode::Event::EnterField(p)))),
        );
        result.add_modal_keys(VI_MODAL_KEYS);
        result.add_modal_keys(ARROW_MODAL_KEYS);
        result.extend(SPECIAL_KEYS.iter().copied());
        result
    }
}

impl Keymap {
    fn get(&self, key: sdl2::keyboard::Keycode) -> Option<Event> {
        self.0.get(&key).copied()
    }

    fn extend(&mut self, src: impl IntoIterator<Item = (Keycode, Event)>) {
        self.0.extend(src);
    }

    fn add_modal_keys<'a>(&mut self, src: impl IntoIterator<Item = &'a (Keycode, mode::Event)>) {
        self.extend(src.into_iter().copied().map(|(k, e)| (k, Event::modal(e))));
    }
}

/// Mapping from digit keycodes to their corresponding digits.
const DIGITS: &[(Keycode, u8)] = &[
    (Keycode::Num0, 0),
    (Keycode::Num1, 1),
    (Keycode::Num2, 2),
    (Keycode::Num3, 3),
    (Keycode::Num4, 4),
    (Keycode::Num5, 5),
    (Keycode::Num6, 6),
    (Keycode::Num7, 7),
    (Keycode::Num8, 8),
    (Keycode::Num9, 9),
];

const SPECIAL_KEYS: &[(Keycode, Event)] = &[
    (
        Keycode::Z,
        Event::Presenter(presenter::Event::Action(session::Action::NewRun(
            session::action::OldDestination::Save,
        ))),
    ),
    (Keycode::Escape, Event::Presenter(presenter::Event::Quit)),
];

/// Mapping from position keycodes to their positions.
const POSITION_KEYS: &[(Keycode, time::Position)] = &[
    // H is reserved for vi.
    (Keycode::O, time::Position::Hours),
    (Keycode::M, time::Position::Minutes),
    (Keycode::S, time::Position::Seconds),
    // M is reserved for minutes.
    (Keycode::Period, time::Position::Milliseconds),
];

/// Mapping from vi-style keys to their default bindings.
const VI_MODAL_KEYS: &[(Keycode, mode::Event)] = &[
    // Home-row
    (Keycode::H, mode::Event::Undo),
    (Keycode::J, mode::Event::Cursor(cursor::Motion::Down)),
    (Keycode::K, mode::Event::Cursor(cursor::Motion::Up)),
    (Keycode::L, mode::Event::Commit),
    // X is delete in vi, so it gets a similar semantics.
    (Keycode::X, mode::Event::Delete),
];

/// Mapping from arrow-style keys to their default bindings.
///
/// These generally just reflect the corresponding vi bindings.
const ARROW_MODAL_KEYS: &[(Keycode, mode::Event)] = &[
    // Arrow key bindings
    (Keycode::Left, mode::Event::Undo),
    (Keycode::Down, mode::Event::Cursor(cursor::Motion::Down)),
    (Keycode::Up, mode::Event::Cursor(cursor::Motion::Up)),
    (Keycode::Right, mode::Event::Commit),
    // Extra bindings for moving the cursor down
    (Keycode::Space, mode::Event::Cursor(cursor::Motion::Down)),
    (Keycode::Return, mode::Event::Cursor(cursor::Motion::Down)),
    // Other modal bindings
    (Keycode::Backspace, mode::Event::Edit(Edit::Remove)),
    (Keycode::Delete, mode::Event::Delete),
];

#[cfg(test)]
mod test {
    use super::*;

    /// Checks a couple of vi keys in the default keymap to make sure the map is being populated
    /// correctly.
    #[test]
    fn default_keymap_vi() {
        let kmap = Keymap::default();

        assert_key(&kmap, Keycode::H, Event::modal(mode::Event::Undo));
        assert_key(&kmap, Keycode::J, Event::motion(cursor::Motion::Down));
        assert_key(&kmap, Keycode::K, Event::motion(cursor::Motion::Up));
        assert_key(&kmap, Keycode::L, Event::modal(mode::Event::Commit));
    }

    fn assert_key(kmap: &Keymap, key: Keycode, evt: Event) {
        assert_eq!(Some(evt), kmap.get(key));
    }

    /// Checks that a representative SDL key-down event gets handled correctly.
    #[test]
    fn from_sdl_default_keymap() {
        let event = sdl2::event::Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(Keycode::Escape),
            scancode: None,
            keymod: sdl2::keyboard::Mod::empty(),
            repeat: false,
        };
        assert_eq!(
            Some(Event::Presenter(presenter::Event::Quit)),
            from_sdl(&event, &Keymap::default())
        );
    }
}
