//! The SDL low-level graphics rendering layer.

use std::{cell::RefMut, rc::Rc};

use super::super::{
    view::gfx::{
        colour,
        font::{self, metrics::TextSizer},
        metrics, pen, render,
    },
    Error, Result,
};
use sdl2::{
    rect::{Point, Rect},
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
        self.pos = Point::new(pos.x, pos.y);
    }

    fn fill(&mut self, rect: metrics::Rect) -> Result<()> {
        let rect = self.convert_rect(rect);
        self.set_screen_bg();
        self.screen.fill_rect(rect).map_err(Error::Blit)?;
        Ok(())
    }

    fn set_font(&mut self, font: font::Id) {
        self.pen.set_font(font, &self.font_manager);
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
        let w = -self.span_w_str(str);
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
}

impl<'a> font::metrics::TextSizer for Renderer<'a> {
    fn span_w(&self, size: i32) -> i32 {
        self.pen.span_w(size)
    }

    fn span_h(&self, size: i32) -> i32 {
        self.pen.span_h(size)
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
        let pen = pen::Pen::new(&font_manager);
        Self {
            screen,
            w_metrics,
            pen,
            font_manager,
            colour_set,
            pos: Point::new(0, 0),
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

    fn put_str_at(&mut self, mut pos: Point, str: &str) -> Result<()> {
        let texture = self.font_texture()?;

        for byte in str.as_bytes() {
            self.put_byte(&texture, *byte, pos)?;
            pos.x += self.span_w(1);
        }

        Ok(())
    }

    fn put_byte<'b>(
        &'b mut self,
        texture: &'b Texture<'a>,
        byte: u8,
        top_left: Point,
    ) -> Result<()> {
        let src = self.font_rect(byte);
        let dst = self.char_rect(top_left);
        self.screen.copy(texture, src, dst).map_err(Error::Blit)
    }

    /// Produces a rectangle with top-left `top_left` and the size of one font
    /// character.
    #[must_use]
    fn char_rect(&self, top_left: Point) -> Rect {
        let char = self.pen.font_metrics().char;
        Rect::new(top_left.x, top_left.y, u32::from(char.w), u32::from(char.h))
    }

    /// Produces the appropriate rectangle for looking up `char` in the font.
    #[must_use]
    fn font_rect(&self, char: u8) -> Rect {
        let metrics = self.pen.font_metrics();
        self.char_rect(Point::new(metrics.glyph_x(char), metrics.glyph_y(char)))
    }
}