//! Mapping between SDL events and presenter events.

use super::super::{
    event::{self, Event},
    presenter::{
        self,
        event::{Edit, Modal},
        state::cursor,
    },
    view,
};
use crate::model::{attempt, timing::time};

/// Wrapper over SDL event pumps to promote them into `event::Pump` instances.
pub struct Pump(pub sdl2::EventPump);

impl event::Pump for Pump {
    fn pump(&mut self) -> Vec<Event> {
        self.0.poll_iter().filter_map(|x| from_sdl(&x)).collect()
    }
}

/// Maps an event from SDL into [Event].
#[must_use]
fn from_sdl(e: &sdl2::event::Event) -> Option<Event> {
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
        Keycode::Backspace => Some(Event::modal(Modal::Edit(Edit::Remove))),
        // Time fields
        // We don't allow entering hours yet, but this may change.
        Keycode::M => Some(Event::modal(Modal::EnterField(time::Position::Minutes))),
        Keycode::S => Some(Event::modal(Modal::EnterField(time::Position::Seconds))),
        Keycode::Period => Some(Event::modal(Modal::EnterField(
            time::Position::Milliseconds,
        ))),
        // Cursor motions
        Keycode::J | Keycode::Down | Keycode::Space | Keycode::Return => {
            Some(Event::modal(Modal::Cursor(cursor::Motion::Down)))
        }
        Keycode::K | Keycode::Up => Some(Event::modal(Modal::Cursor(cursor::Motion::Up))),
        // Top-level commands
        Keycode::H | Keycode::Left => Some(Event::modal(Modal::Undo)),
        Keycode::L | Keycode::Right => Some(Event::modal(Modal::Commit)),
        Keycode::X | Keycode::Delete => Some(Event::modal(Modal::Delete)),
        Keycode::Z => Some(Event::Presenter(presenter::event::Event::Action(
            // TODO(@MattWindsor91): way of discarding runs
            attempt::Action::NewRun(attempt::action::OldDestination::Save),
        ))),
        Keycode::Escape => Some(Event::Presenter(presenter::event::Event::Quit)),
        _ => None,
    }
}
