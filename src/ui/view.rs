//! The visual portion of the zombiesplit user interface.
pub mod config;
pub mod gfx;
mod widget;

use crate::ui::view::widget::Widget;

use self::gfx::{metrics, render::Renderer};

use super::{presenter, Result};

pub use config::Config;

/// The top-level view structure.
pub struct View<R> {
    /// The renderer to use for the view.
    renderer: R,
    /// The root widget of the user interface.
    root: widget::Root,
}

impl<R: Renderer> View<R> {
    /// Creates a new graphics core.
    #[must_use]
    pub fn new(renderer: R, wmetrics: gfx::metrics::Window) -> Self {
        let bounds = metrics::Rect {
            top_left: metrics::Point { x: 0, y: 0 },
            size: metrics::Size {
                w: wmetrics.win_w,
                h: wmetrics.win_h,
            },
        };
        let ctx = widget::LayoutContext { wmetrics, bounds };

        Self {
            renderer,
            root: widget::Root::new(ctx),
        }
    }

    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &presenter::State) -> Result<()> {
        self.renderer.clear();
        self.redraw_widgets(state)?;
        self.renderer.present();

        Ok(())
    }

    fn redraw_widgets(&mut self, state: &presenter::State) -> Result<()> {
        let mut widgets: Vec<&dyn Widget<presenter::State>> = vec![&mut self.root];
        while let Some(w) = widgets.pop() {
            w.render(&mut self.renderer, state)?;
            widgets.append(&mut w.children());
        }
        Ok(())
    }
}
