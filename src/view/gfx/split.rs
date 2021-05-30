//! Logic for drawing splits.

use std::convert::TryFrom;

use crate::{model,
    presenter::{split, Presenter},
    view::error::Result
};
use super::{colour, render, metrics};

/// Views splits by drawing them using the given renderer.
pub struct View<'a> {
    /// The renderer used to draw primitives.
    renderer: &'a mut dyn render::Renderer,
    /// The height of one split.
    split_h: i32,
}

impl<'a> View<'a> {
    /// Creates a new view using the given renderer and split height.
    pub fn new(renderer: &'a mut dyn render::Renderer, split_h: i32) -> Self {
        View{renderer, split_h}
    }

    /// Draws the splits of `state`
    pub fn draw(&mut self, state: &Presenter) -> Result<()> {
        for split in state.splits() {
            self.renderer.set_pos(0, self.split_y(split));
            self.draw_split(split)?;
        }
        Ok(())
    }

    fn split_y(&self, split: split::Ref) -> i32 {
        i32::try_from(split.index).unwrap_or_default() * self.split_h
    }

    fn draw_split(&mut self, split: split::Ref) -> Result<()> {
        self.draw_name(split)?;
        self.draw_time(split)?;
        Ok(())
    }

    fn draw_name(&mut self, split: split::Ref) -> Result<()> {
        let colour = colour::Key::Name(split.position());
        self.renderer.set_font(render::FontId::Normal(colour));
        self.renderer.put_str(&split.split.name)?;
        Ok(())
    }

    fn draw_time(&mut self, split: split::Ref) -> Result<()> {
        // TODO(@MattWindsor91): decouple this.
        self.renderer
            .set_x(metrics::WINDOW.split_time_x(metrics::FONT));
        if split.split.has_times() {
            self.draw_summed_time(split.split.summed_time())
        } else {
            self.draw_time_placeholder()
        }
    }

    fn draw_summed_time(&mut self, time: model::time::Time) -> Result<()> {
        let colour = colour::Key::RunAhead; // for now

        // TODO(@MattWindsor91): hours?
        self.renderer.set_font(render::FontId::Normal(colour));
        self.renderer.put_str(&time_str(time))
    }

    fn draw_time_placeholder(&mut self) -> Result<()> {
        self.renderer.set_font(render::FontId::Normal(colour::Key::NoTime));
        self.renderer.put_str("--'--\"---")
    }
}

#[must_use]
pub fn time_str(time: model::time::Time) -> String {
    format!("{}'{}\"{}", time.mins, time.secs, time.millis)
}
