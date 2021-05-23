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
        let tl = metrics::split_name_top_left(split.index);
        let colour = colour::Key::Name(split.position());
        self.renderer
            .put_str(&split.split.name, tl, render::FontId::Normal(colour))
    }

    fn draw_split_time(&mut self, split: state::SplitRef) -> Result<()> {
        let tl = metrics::split_time_top_left(split.index);
        self.draw_split_time_placeholder(tl)?;
        if split.split.has_times() {
            self.draw_split_summed_time(tl, split.split.summed_time())
        } else {
            self.draw_split_time_placeholder(tl)
        }
    }

    fn draw_split_summed_time(
        &mut self,
        tl: sdl2::rect::Point,
        time: model::time::Time,
    ) -> Result<()> {
        let colour = Key::RunAhead; // for now

        // TODO(@MattWindsor91): hours?
        let time_str = format!("{}'{}\"{}", time.mins, time.secs, time.millis);
        self.renderer
            .put_str(&time_str, tl, render::FontId::Normal(colour))
    }

    fn draw_split_time_placeholder(&mut self, tl: sdl2::rect::Point) -> Result<()> {
        self.renderer
            .put_str("--'--\"---", tl, render::FontId::Normal(Key::NoTime))
    }

    /// Draws any editor required by the current state.
    fn draw_editor(&mut self, state: &state::State) -> Result<()> {
        if let state::Action::Entering(ref editor) = state.action {
            let tl = metrics::split_time_top_left(state.cursor);
            let tl = metrics::offset(tl, field_char_offset(editor.position()), 0);
            self.renderer
                .put_str(&editor.to_string(), tl, render::FontId::Normal(Key::Editor))
        } else {
            Ok(())
        }
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
        .window("zombiesplit", metrics::WIN_W, metrics::WIN_H)
        .position_centered()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
