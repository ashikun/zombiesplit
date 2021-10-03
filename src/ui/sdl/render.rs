//! The SDL low-level graphics rendering layer.

use std::{cell::RefMut, rc::Rc};

use super::super::{
    view::gfx::{
        colour, font,
        metrics::{self, Point},
        pen, render,
    },
    Error, Result,
};
use sdl2::{
    render::{Canvas, Texture},
    video,
};

/// The SDL window graphics renderer.
pub struct Renderer<'a> {
    /// The target screen canvas.
    screen: RefMut<'a, Canvas<video::Window>>,
    /// The current window metrics.
    w_metrics: metrics::Window,
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
    fn size(&self) -> metrics::Size {
        metrics::Size {
            w: self.w_metrics.win_w,
            h: self.w_metrics.win_h,
        }
    }

    fn set_pos(&mut self, pos: metrics::Point) {
        self.pos = pos;
    }

    fn fill(&mut self, rect: metrics::Rect) -> Result<()> {
        let rect = self.convert_rect(rect);
        self.set_screen_bg();
        self.screen.fill_rect(rect).map_err(Error::Blit)?;
        Ok(())
    }

    fn set_font(&mut self, font: font::Id) {
        self.pen.set_font(font, &self.font_manager.metrics_set);
    }

    fn set_bg_colour(&mut self, colour: colour::bg::Id) {
        self.pen.bg_colour = colour;
    }

    fn set_fg_colour(&mut self, colour: colour::fg::Id) {
        self.pen.fg_colour = colour;
    }

    fn put_str(&mut self, str: &str) -> Result<()> {
        self.put_str_at(self.pos, str)
    }

    fn put_str_r(&mut self, str: &str) -> Result<()> {
        let w = -self.font_metrics().span_w_str(str);
        self.put_str_at(self.pos.offset(w, 0), str)
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

    fn font_metrics(&self) -> &font::Metrics {
        self.pen.font_metrics()
    }
}

impl<'a> Renderer<'a> {
    /// Constructs a [Renderer] using the given screen, metrics, and font manager.
    #[must_use]
    pub fn new(
        screen: RefMut<'a, Canvas<video::Window>>,
        w_metrics: metrics::Window,
        font_manager: super::font::Manager<'a>,
        colour_set: &'a colour::Set,
    ) -> Self {
        let pen = pen::Pen::new(&font_manager.metrics_set);
        Self {
            screen,
            w_metrics,
            pen,
            font_manager,
            colour_set,
            pos: Point::default(),
        }
    }

    // Sets the screen draw colour to the background colour.
    fn set_screen_bg(&mut self) {
        let colour = self.colour_set.bg.get(self.pen.bg_colour);
        self.screen.set_draw_color(colour);
    }

    fn font_texture(&mut self) -> Result<Rc<Texture<'a>>> {
        Ok(self.font_manager.texture(self.font_spec())?)
    }

    fn font_spec(&self) -> font::Spec {
        self.pen.font_spec()
    }

    fn convert_rect(&self, rect: metrics::Rect) -> sdl2::rect::Rect {
        let pos = self.pos.offset(rect.top_left.x, rect.top_left.y);
        sdl2::rect::Rect::new(pos.x, pos.y, rect.size.w, rect.size.h)
    }

    fn put_str_at(&mut self, pos: Point, str: &str) -> Result<()> {
        let texture = self.font_texture()?;

        for glyph in self.pen.font_metrics().layout_str(pos, str) {
            let src = super::metrics::convert_rect(&glyph.src);
            let dst = super::metrics::convert_rect(&glyph.dst);

            self.screen.copy(&texture, src, dst).map_err(Error::Blit)?;
        }

        Ok(())
    }
}
