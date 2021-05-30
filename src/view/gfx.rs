//! Graphics rendering.

pub mod colour;
pub mod metrics; // for now
pub mod render;
mod split;

use crate::{
    model::time::position,
    presenter::{
        editor::{self, Editor},
        Presenter,
    },
};

use self::colour::Key;

use super::error::{Error, Result};

use render::{Renderer, Region};

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
        let mut region = 
            Region{
                renderer: &mut self.renderer,
                x: metrics::WINDOW.split_xpad,
                y: metrics::WINDOW.split_ypos
            };
        split::View::new(&mut region, metrics::WINDOW.split_h).draw(state)
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
        self.renderer
            .set_pos(
                metrics::WINDOW.split_time_x(metrics::FONT),
                metrics::WINDOW.split_y(editor.cur.position()),
            );
        self.renderer.set_font(render::FontId::Normal(Key::Editor));
        self.renderer.put_str(&split::time_str(editor.time))
    }

    fn draw_editor_field(&mut self, field: &editor::Field) -> Result<()> {
        self.renderer.move_chars(field_char_offset(field.position()), 0);
        self.renderer.set_font(render::FontId::Normal(Key::FieldEditor));
        self.renderer.put_str(&field.to_string())
    }
}

fn field_char_offset(field: position::Name) -> i32 {
    match field {
        // Hours not supported yet.
        position::Name::Hours | position::Name::Minutes => 0,
        position::Name::Seconds => 3,
        position::Name::Milliseconds => 6,
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
