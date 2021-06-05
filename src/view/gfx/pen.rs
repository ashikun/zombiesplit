//! The [Pen] struct and implementations.

use super::{colour, font};

/// The pen used for rendering.
pub struct Pen {
    /// The current font ID.
    font: font::Id,
    /// The metrics of the current font.
    f_metrics: font::Metrics,
    /// The current foreground colour.
    fg_colour: colour::Key,
}

impl Pen {
    /// Creates a new pen with the default font and colours.
    #[must_use]
    pub fn new(metrics: &dyn font::metrics::Source<font::Id>) -> Self {
        let font = font::Id::Normal;
        Self {
            font,
            f_metrics: metrics.metrics(font),
            fg_colour: colour::Key::NoTime,
        }
    }

    /// Sets this pen's font, also recording the font metrics in the pen.
    pub fn set_font(&mut self, font: font::Id, metrics: &dyn font::metrics::Source<font::Id>) {
        self.font = font;
        self.f_metrics = metrics.metrics(self.font);
    }

    /// Sets this pen's foreground colour.
    pub fn set_fg_colour(&mut self, id: colour::Key) {
        self.fg_colour = id;
    }

    /// Gets the pen's current font spec.
    #[must_use]
    pub fn font_spec(&self) -> font::manager::Spec {
        font::manager::Spec {
            id: self.font,
            colour: self.fg_colour,
        }
    }

    /// Gets the pen's current font metrics.
    #[must_use]
    pub fn font_metrics(&self) -> font::Metrics {
        self.f_metrics
    }
}
