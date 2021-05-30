//! Graphics rendering.

pub mod colour;
pub mod metrics; // for now
mod position;
pub mod render;
mod split;

use crate::{
    model::time,
    presenter::{
        editor::{self, Editor},
        Presenter,
    },
};

use self::colour::Key;

use super::error::{Error, Result};

use position::{Position, X, Y};
use render::{Region, Renderer};

pub struct Core<'a> {
    pub renderer: render::Window<'a>,
}

impl<'a> Core<'a> {
    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &Presenter) -> Result<()> {
        self.renderer.clear();

        self.draw_splits(state)?;

        if let Some(ref editor) = state.editor() {
            self.draw_editor(editor)?;
        }

        self.renderer.present();

        Ok(())
    }

    fn draw_splits(&mut self, state: &Presenter) -> Result<()> {
        let mut region = Region {
            renderer: &mut self.renderer,
            rect: metrics::WINDOW.splits_rect(),
        };
        split::View::new(&mut region, metrics::sat_i32(metrics::WINDOW.split_h)).draw(state)
    }

    /// Draws any editor required by the current state.
    fn draw_editor(&mut self, editor: &Editor) -> Result<()> {
        self.draw_editor_time(editor)?;
        if let Some(ref f) = editor.field {
            self.draw_editor_field(f)?;
        };
        Ok(())
    }

    fn draw_editor_time(&mut self, editor: &Editor) -> Result<()> {
        self.move_to_editor();
        self.renderer.set_font(render::FontId::Normal(Key::Editor));
        self.renderer.put_str(&split::time_str(editor.time))
    }

    fn move_to_editor(&mut self) {
        // TODO(@MattWindsor91): fix editor position.
        self.renderer.set_pos(Position {
            x: X::Right(0),
            y: Y::Bottom(0),
        });
        self.renderer.move_chars(-metrics::TIME_CHARS, -1);
    }

    fn draw_editor_field(&mut self, field: &editor::Field) -> Result<()> {
        self.renderer
            .move_chars(field_char_offset(field.position()), 0);
        self.renderer
            .set_font(render::FontId::Normal(Key::FieldEditor));
        self.renderer.put_str(&field.to_string())
    }
}

fn field_char_offset(field: time::position::Name) -> i32 {
    match field {
        // Hours not supported yet.
        time::position::Name::Hours | time::position::Name::Minutes => 0,
        time::position::Name::Seconds => 3,
        time::position::Name::Milliseconds => 6,
    }
}

/// Makes a zombiesplit window.
///
/// # Errors
///
/// Returns an error if SDL fails to make the window.
pub fn make_window(video: &sdl2::VideoSubsystem) -> Result<sdl2::video::Window> {
    let window = video
        .window("zombiesplit", metrics::WINDOW.win_w, metrics::WINDOW.win_h)
        .position_centered()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
