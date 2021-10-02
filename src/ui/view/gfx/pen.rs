//! The [Pen] struct and implementations.

use super::{colour, font};

/// The pen used for rendering.
pub struct Pen {
    /// The current font ID.
    font: font::Id,
    /// The metrics of the current font.
    f_metrics: font::Metrics,
    /// The current foreground colour.
    pub fg_colour: colour::fg::Id,
    /// The current background colour.
    pub bg_colour: colour::bg::Id,
}

impl Pen {
    /// Creates a new pen with the default font and colours.
    #[must_use]
    pub fn new(metrics: &font::Map<font::Metrics>) -> Self {
        let font = font::Id::Medium;
        Self {
            font,
            f_metrics: metrics[font],
            fg_colour: colour::fg::Id::NoTime,
            bg_colour: colour::bg::Id::Window,
        }
    }

    /// Sets this pen's font, also recording the font metrics in the pen.
    pub fn set_font(&mut self, font: font::Id, metrics: &font::Map<font::Metrics>) {
        self.font = font;
        self.f_metrics = metrics[self.font];
    }

    /// Gets the pen's current font spec.
    #[must_use]
    pub fn font_spec(&self) -> font::Spec {
        font::Spec {
            id: self.font,
            colour: self.fg_colour,
        }
    }

    /// Gets the pen's current font metrics.
    #[must_use]
    pub fn font_metrics(&self) -> &font::Metrics {
        &self.f_metrics
    }
}
