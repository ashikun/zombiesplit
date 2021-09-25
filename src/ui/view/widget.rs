/*! Widgets.

The (reference) UI for zombiesplit contains several self-rendering widgets,
each of which has access to the presenter state and a renderer.
*/

mod editor;
mod header;
mod split;
mod total;

use super::{
    super::presenter::State,
    error::Result,
    gfx::{metrics, render},
};

/// Trait for things that can render information from a presenter.
pub trait Widget<State> {
    /// Renders information from `s` onto the renderer `r`.
    fn render(&mut self, r: &mut dyn render::Renderer, s: &State) -> Result<()>;
}

/// A collection of widgets, combined with their renderer.
pub struct Set<'a> {
    renderer: render::Window<'a>,
    widgets: Vec<Box<dyn Widget<State>>>,
}

impl<'a> Set<'a> {
    /// Creates a new graphics core.
    #[must_use]
    pub fn new(renderer: render::Window<'a>, wmetrics: metrics::Window) -> Self {
        Self {
            renderer,
            widgets: make_widgets(wmetrics),
        }
    }

    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &State) -> Result<()> {
        self.renderer.clear();

        for w in &mut self.widgets {
            w.render(&mut self.renderer, state)?;
        }

        self.renderer.present();

        Ok(())
    }
}

fn make_widgets(wmetrics: metrics::Window) -> Vec<Box<dyn Widget<State>>> {
    vec![
        make_splits(wmetrics),
        make_header(wmetrics),
        make_total(wmetrics),
    ]
}

fn make_splits(wmetrics: metrics::Window) -> Box<dyn Widget<State>> {
    Box::new(split::Widget::new(
        wmetrics.splits_rect(),
        metrics::sat_i32(wmetrics.split_h),
    ))
}

fn make_header(wmetrics: metrics::Window) -> Box<dyn Widget<State>> {
    Box::new(header::Widget {
        rect: wmetrics.header_rect(),
    })
}

fn make_total(wmetrics: metrics::Window) -> Box<dyn Widget<State>> {
    Box::new(total::Widget {
        rect: wmetrics.total_rect(),
    })
}
