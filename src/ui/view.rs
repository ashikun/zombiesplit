//! The visual portion of the zombiesplit user interface.
use layout::Layoutable;
use widget::{Root, Widget};

use super::{presenter, Result};

pub use self::config::Config;
use self::gfx::Renderer;

pub mod config;
pub mod event;
pub mod gfx;
mod layout;
mod widget;

pub use event::Event;

/// The top-level view structure.
///
/// This has a lifetime dependency on the view configuration.
pub struct View<'c, R> {
    /// The renderer to use for the view.
    renderer: R,
    /// The root widget of the user interface.
    root: Root,
    /// The user layout configuration.
    config: &'c config::Layout,
}

impl<'c, R: Renderer> View<'c, R> {
    /// Creates a new graphics core.
    #[must_use]
    pub fn new(renderer: R, config: &'c config::layout::Layout) -> Self {
        let mut result = Self {
            renderer,
            root: Root::new(&config.widgets),
            config,
        };
        result.layout_root(config.window.size);
        result
    }

    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &presenter::State) -> Result<()> {
        self.renderer.clear(gfx::colour::bg::Id::Window)?;
        self.root.render(&mut self.renderer, state)?;
        self.renderer.present();

        Ok(())
    }

    /// Handles the event `event`.
    pub fn handle_event(&mut self, event: &event::Event) {
        match event {
            Event::Resize(size) => self.layout_root(*size),
        }
    }

    fn layout_root(&mut self, size: ugly::metrics::Size) {
        self.root.layout(layout::Context {
            config: self.config,
            bounds: ugly::metrics::Rect {
                top_left: ugly::metrics::Point::default(),
                size,
            },
            font_metrics: self.renderer.font_metrics(),
        });
    }
}
