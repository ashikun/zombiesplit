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

/// Type alias for keymaps.
pub type Keymap = HashMap<Keycode, Event>;

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
            keymap: default_keymap(),
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
        } => keymap.get(k).copied(),
        _ => None,
    }
}

/*
fn from_key(k: sdl2::keyboard::Keycode) -> Option<Event> {

    use crate::ui::presenter::mode;
    use sdl2::keyboard::Keycode;
    if let Some(digit) = key_to_digit(k) {
        Some(Event::digit(digit))
    } else {
        match k {
            // Editing
            // Time fields
            // We don't allow entering hours yet, but this may change.
            // Cursor motions
            Keycode::J | Keycode::Down | Keycode::Space | Keycode::Return => {
                Some(Event::modal(Modal::Cursor(cursor::Motion::Down)))
            }
            Keycode::K | Keycode::Up => Some(Event::modal(Modal::Cursor(cursor::Motion::Up))),
            // Top-level commands
            Keycode::H | Keycode::Left => Some(Event::modal(Modal::Undo)),
            Keycode::L | Keycode::Right => Some(Event::modal(Modal::Commit)),
            Keycode::X | Keycode::Delete => Some(Event::modal(Modal::Delete)),

        }
    }
}
 */

fn default_keymap() -> HashMap<sdl2::keyboard::Keycode, Event> {
    let mut result = HashMap::new();
    result.extend(DIGITS.iter().copied().map(|(k, d)| (k, Event::digit(d))));
    result.extend(
        POSITION_KEYS
            .iter()
            .copied()
            .map(|(k, p)| (k, Event::modal(mode::Event::EnterField(p)))),
    );
    add_modal_keys(&mut result, VI_MODAL_KEYS);
    add_modal_keys(&mut result, ARROW_MODAL_KEYS);
    result.extend(SPECIAL_KEYS.iter().copied());
    result
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

fn add_modal_keys<'a>(dst: &mut Keymap, src: impl IntoIterator<Item = &'a (Keycode, mode::Event)>) {
    dst.extend(src.into_iter().copied().map(|(k, e)| (k, Event::modal(e))));
}

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
