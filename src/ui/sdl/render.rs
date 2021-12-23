//! The SDL low-level graphics rendering layer.

use std::cell::RefMut;

use super::super::view::gfx::{
    colour, font,
    metrics::{self, Point},
    render, Error, Result,
};
use crate::ui::view::gfx::font::Spec;
use sdl2::{render::Canvas, video};

/// The SDL window graphics renderer.
pub struct Renderer<'a> {
    /// The target screen canvas.
    screen: RefMut<'a, Canvas<video::Window>>,
    /// The font manager.
    font_manager: super::font::Manager<'a>,
    /// The colour set.
    colour_set: &'a colour::Set,
}

impl<'a> render::Renderer for Renderer<'a> {
    fn write(&mut self, mut pos: Point, font: Spec, s: &str) -> Result<Point> {
        let texture = self.font_manager.texture(font)?;
        let fm = &self.font_manager.metrics_set[font.id];

        for glyph in fm.layout_str(pos, s.as_bytes()) {
            let src = super::metrics::convert_rect(&glyph.src);
            let dst = super::metrics::convert_rect(&glyph.dst);

            // Move from the end of the last character to the start of the next one.
            pos = glyph.dst.point(fm.pad.w, 0, metrics::Anchor::TOP_RIGHT);

            self.screen.copy(&texture, src, dst).map_err(Error::Blit)?;
        }

        Ok(pos)
    }

    fn fill(&mut self, rect: metrics::Rect, colour: colour::bg::Id) -> Result<()> {
        let rect = super::metrics::convert_rect(&rect);
        self.set_screen_bg(colour);
        self.screen.fill_rect(rect).map_err(Error::Blit)
    }

    /// Clears the screen.
    fn clear(&mut self) {
        self.set_screen_bg(colour::bg::Id::Window);
        self.screen.clear();
    }

    /// Refreshes the screen.
    fn present(&mut self) {
        self.screen.present();
    }

    fn font_metrics(&self) -> &font::Map<font::Metrics> {
        &self.font_manager.metrics_set
    }
}

impl<'a> Renderer<'a> {
    /// Constructs a [Renderer] using the given screen, font manager, and colour set.
    #[must_use]
    pub fn new(
        screen: RefMut<'a, Canvas<video::Window>>,
        font_manager: super::font::Manager<'a>,
        colour_set: &'a colour::Set,
    ) -> Self {
        Self {
            screen,
            font_manager,
            colour_set,
        }
    }

    // Sets the screen draw colour to `bg`.
    fn set_screen_bg(&mut self, bg: colour::bg::Id) {
        let colour = self.colour_set.bg.get(bg);
        self.screen.set_draw_color(colour_to_sdl(colour));
    }
}

/// Converts a zombiesplit colour to a SDL one.
fn colour_to_sdl(c: colour::definition::Colour) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(c.red_byte(), c.green_byte(), c.blue_byte(), c.alpha_byte())
}
