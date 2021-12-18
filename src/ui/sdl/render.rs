//! The SDL low-level graphics rendering layer.

use std::{cell::RefMut, rc::Rc};

use super::super::view::gfx::{
    colour, font,
    metrics::{self, Point},
    pen, render, Error, Result,
};
use crate::ui::view::gfx::font::Spec;
use sdl2::{
    render::{Canvas, Texture},
    video,
};

/// The SDL window graphics renderer.
pub struct Renderer<'a> {
    /// The target screen canvas.
    screen: RefMut<'a, Canvas<video::Window>>,
    /// The font manager.
    font_manager: super::font::Manager<'a>,
    /// The pen.
    pen: pen::Pen,
    /// The colour set.
    colour_set: &'a colour::Set,
    /// The current position.
    pos: Point,
}

impl<'a> render::Renderer for Renderer<'a> {
    fn write(&mut self, pos: Point, font: Spec, s: &str) -> Result<Point> {
        self.pos = pos;
        self.pen.fg_colour = font.colour;
        self.pen.set_font(font.id, &self.font_manager.metrics_set);
        self.put_str_at(pos, s.as_bytes())
    }

    fn set_bg_colour(&mut self, colour: colour::bg::Id) {
        self.pen.bg_colour = colour;
    }

    fn fill(&mut self, rect: metrics::Rect) -> Result<()> {
        let rect = self.convert_rect(rect);
        self.set_screen_bg();
        self.screen.fill_rect(rect).map_err(Error::Blit)
    }

    /// Clears the screen.
    fn clear(&mut self) {
        render::Renderer::set_bg_colour(self, colour::bg::Id::Window);
        self.set_screen_bg();
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
        let pen = pen::Pen::new(&font_manager.metrics_set);
        Self {
            screen,
            pen,
            font_manager,
            colour_set,
            pos: Point::default(),
        }
    }

    // Sets the screen draw colour to the background colour.
    fn set_screen_bg(&mut self) {
        let colour = self.colour_set.bg.get(self.pen.bg_colour);
        self.screen.set_draw_color(colour_to_sdl(colour));
    }

    fn font_spec(&self) -> font::Spec {
        self.pen.font_spec()
    }

    fn convert_rect(&self, rect: metrics::Rect) -> sdl2::rect::Rect {
        let pos = self.pos.offset(rect.top_left.x, rect.top_left.y);
        sdl2::rect::Rect::new(pos.x, pos.y, rect.size.w, rect.size.h)
    }

    fn put_str_at(&mut self, mut pos: Point, str: &[u8]) -> Result<Point> {
        let texture = self.font_texture()?;

        for glyph in self.pen.font_metrics().layout_str(pos, str) {
            let src = super::metrics::convert_rect(&glyph.src);
            let dst = super::metrics::convert_rect(&glyph.dst);

            // Move from the end of the last character to the start of the next one.
            pos = glyph.dst.point(
                self.pen.font_metrics().pad.w_i32(),
                0,
                metrics::Anchor::TOP_RIGHT,
            );

            self.screen.copy(&texture, src, dst).map_err(Error::Blit)?;
        }

        Ok(pos)
    }

    fn font_texture(&mut self) -> Result<Rc<Texture<'a>>> {
        Ok(self.font_manager.texture(self.font_spec())?)
    }
}

/// Converts a zombiesplit colour to a SDL one.
fn colour_to_sdl(c: colour::definition::Colour) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(c.red_byte(), c.green_byte(), c.blue_byte(), c.alpha_byte())
}
