//! Graphics rendering.

pub mod colour;
pub mod metrics; // for now
pub mod render;

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
        self.renderer
            .put_str("--'--\"---", tl, render::FontId::Normal(Key::NoTime))
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
