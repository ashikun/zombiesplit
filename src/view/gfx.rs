//! Graphics rendering.

pub mod colour;
mod editor;
pub mod font;
mod header;
pub mod metrics; // for now
mod position;
pub mod render;
mod split;
mod widget;

use crate::presenter::Presenter;

use super::error::{Error, Result};

use widget::Widget;

pub struct Core<'a> {
    renderer: render::Window<'a>,
    widgets: Vec<Box<dyn widget::Widget>>,
}

impl<'a> Core<'a> {
    /// Creates a new graphics core.
    #[must_use]
    pub fn new(renderer: render::Window<'a>) -> Self {
        Self {
            renderer,
            widgets: make_widgets(metrics::WINDOW),
        }
    }

    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &Presenter) -> Result<()> {
        self.renderer.clear();

        for w in &mut self.widgets {
            w.render(&mut self.renderer, state)?;
        }

        self.renderer.present();

        Ok(())
    }
}

fn make_widgets(wmetrics: metrics::Window) -> Vec<Box<dyn Widget>> {
    vec![
        make_splits(wmetrics),
        make_header(wmetrics),
        make_editor(wmetrics),
    ]
}

fn make_splits(wmetrics: metrics::Window) -> Box<dyn Widget> {
    Box::new(split::Widget::new(
        wmetrics.splits_rect(),
        metrics::sat_i32(wmetrics.split_h),
    ))
}

fn make_header(wmetrics: metrics::Window) -> Box<dyn Widget> {
    Box::new(header::Widget {
        rect: wmetrics.header_rect(),
    })
}

fn make_editor(wmetrics: metrics::Window) -> Box<dyn Widget> {
    Box::new(editor::Widget::new(wmetrics.editor_rect()))
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
