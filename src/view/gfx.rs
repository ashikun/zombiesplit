//! Graphics rendering.

pub mod colour;
pub mod metrics; // for now
pub mod render;

use crate::{
    model::{self, time::position},
    presenter::{
        editor::{self, Editor},
        split, Presenter,
    },
};

use self::colour::Key;

use super::error::{Error, Result};

pub struct Core<'a> {
    pub renderer: render::Renderer<'a>,
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
        for split in state.splits() {
            self.renderer.set_pos(
                metrics::WINDOW.split_xpad,
                metrics::WINDOW.split_y(split.index),
            );
            self.draw_split(split)?;
        }
        Ok(())
    }

    fn draw_split(&mut self, split: split::Ref) -> Result<()> {
        self.draw_split_name(split)?;
        self.draw_split_time(split)?;
        Ok(())
    }

    fn draw_split_name(&mut self, split: split::Ref) -> Result<()> {
        let colour = colour::Key::Name(split.position());
        self.renderer
            .set_font(render::FontId::Normal(colour))
            .put_str(&split.split.name)?;
        Ok(())
    }

    fn draw_split_time(&mut self, split: split::Ref) -> Result<()> {
        self.renderer
            .set_x(metrics::WINDOW.split_time_x(metrics::FONT));
        if split.split.has_times() {
            self.draw_split_summed_time(split.split.summed_time())
        } else {
            self.draw_split_time_placeholder()
        }
    }

    fn draw_split_summed_time(&mut self, time: model::time::Time) -> Result<()> {
        let colour = Key::RunAhead; // for now

        // TODO(@MattWindsor91): hours?
        self.renderer
            .set_font(render::FontId::Normal(colour))
            .put_str(&time_str(time))?;
        Ok(())
    }

    fn draw_split_time_placeholder(&mut self) -> Result<()> {
        self.renderer
            .set_font(render::FontId::Normal(Key::NoTime))
            .put_str("--'--\"---")?;
        Ok(())
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
            )
            .set_font(render::FontId::Normal(Key::Editor))
            .put_str(&time_str(editor.time))?;
        Ok(())
    }

    fn draw_editor_field(&mut self, field: &editor::Field) -> Result<()> {
        self.renderer
            .move_chars(field_char_offset(field.position()), 0)
            .set_font(render::FontId::Normal(Key::FieldEditor))
            .put_str(&field.to_string())?;
        Ok(())
    }
}

fn time_str(time: model::time::Time) -> String {
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
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
