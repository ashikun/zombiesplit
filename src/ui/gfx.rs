//! Graphics rendering.

pub mod colour;
pub mod metrics; // for now
pub mod render;

use crate::model::run;
use std::convert::TryInto;

use super::{
    error::{Error, Result},
    state,
};
use sdl2::rect::Point;

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
        for (num, s) in state.run.splits.iter().enumerate() {
            self.draw_split(state, s, num)?;
        }
        Ok(())
    }

    fn draw_split(&mut self, state: &state::State, split: &run::Split, num: usize) -> Result<()> {
        let tl = split_name_top_left(num);
        let colour = colour::Key::fg_split_text(num, state.cursor);
        self.renderer
            .put_str(&split.name, tl, render::FontId::Normal(colour))
    }
}

fn split_name_top_left(num: usize) -> sdl2::rect::Point {
    let ns: i32 = num.try_into().unwrap_or_default();
    Point::new(4, 4 + (16 * ns))
}


/// Makes a zombiesplit window.
///
/// # Errors
///
/// Returns an error if SDL fails to make the window.
pub fn make_window(video: &sdl2::VideoSubsystem) -> Result<sdl2::video::Window> {
    let window = video
        .window("zombiesplit", 320, 640)
        .position_centered()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
