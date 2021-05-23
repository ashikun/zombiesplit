//! Graphics rendering.

pub mod colour;
pub mod metrics; // for now
pub mod render;

use crate::model::{self, time::position};

use self::colour::Key;

use super::{
    error::{Error, Result},
    state,
};
pub struct Core<'a> {
    pub renderer: render::Renderer<'a>,
}

impl<'a> Core<'a> {
    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &state::State) -> Result<()> {
        self.renderer.clear();

        self.draw_splits(state)?;
        self.draw_editor(state)?;

        self.renderer.present();

        Ok(())
    }

    fn draw_splits(&mut self, state: &state::State) -> Result<()> {
        for split in state.splits() {
            self.renderer.set_pos(
                metrics::WINDOW.split_xpad,
                metrics::WINDOW.split_y(split.index),
            );
            self.draw_split(split)?;
        }
        Ok(())
    }

    fn draw_split(&mut self, split: state::SplitRef) -> Result<()> {
        self.draw_split_name(split)?;
        self.draw_split_time(split)?;
        Ok(())
    }

    fn draw_split_name(&mut self, split: state::SplitRef) -> Result<()> {
        let colour = colour::Key::Name(split.position());
        self.renderer
            .set_font(render::FontId::Normal(colour))
            .put_str(&split.split.name)?;
        Ok(())
    }

    fn draw_split_time(&mut self, split: state::SplitRef) -> Result<()> {
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
        let time_str = format!("{}'{}\"{}", time.mins, time.secs, time.millis);
        self.renderer
            .set_font(render::FontId::Normal(colour))
            .put_str(&time_str)?;
        Ok(())
    }

    fn draw_split_time_placeholder(&mut self) -> Result<()> {
        self.renderer
            .set_font(render::FontId::Normal(Key::NoTime))
            .put_str("--'--\"---")?;
        Ok(())
    }

    /// Draws any editor required by the current state.
    fn draw_editor(&mut self, state: &state::State) -> Result<()> {
        if let state::Action::Entering(ref editor) = state.action {
            self.renderer
                .set_pos(
                    metrics::WINDOW.split_time_x(metrics::FONT),
                    metrics::WINDOW.split_y(state.cursor),
                )
                .move_chars(field_char_offset(editor.position()), 0)
                .set_font(render::FontId::Normal(Key::Editor))
                .put_str(&editor.to_string())?;
        }
        Ok(())
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
