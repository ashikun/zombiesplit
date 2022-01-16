//! The visual portion of the zombiesplit user interface.
use layout::Layoutable;
use widget::Widget;

use super::{presenter, Result};

pub use self::config::Config;
use self::gfx::{font, render::Renderer};

pub mod config;
pub mod gfx;
mod layout;
mod widget;

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
    pub fn new(renderer: R, config: &config::layout::Layout) -> Self {
        let mut root = widget::Root::default();
        root.layout(root_layout_context(renderer.font_metrics(), config));
        Self { renderer, root }
    }

    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &presenter::State) -> Result<()> {
        self.renderer.clear();
        self.root.render(&mut self.renderer, state)?;
        self.renderer.present();

        Ok(())
    }
}

/// Creates the root layout context.
fn root_layout_context<'m>(
    font_metrics: &'m font::Map<font::Metrics>,
    config: &'m config::layout::Layout,
) -> layout::Context<'m> {
    let bounds = config.window.win_rect();

    layout::Context {
        config,
        bounds,
        font_metrics,
    }
}
