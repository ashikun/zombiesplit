//! Font metrics.
use std::collections::HashMap;

use super::super::metrics::{conv::sat_i32, Point, Rect, Size};
use serde::{Deserialize, Serialize};

// We hardcode the general layout of a font texture using the following
// constants:

/// The number of columns in a font.
const NUM_COLS: u8 = 32;

/// A font metrics set.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metrics {
    /// Dimensions of one character in the font, without padding.
    ///
    /// This is the size of one cell in the texture grid, and so will
    pub char: Size,
    /// Dimensions of padding between characters in the font.
    pub pad: Size,
    /// Width overrides for specific characters.
    ///
    /// The font grid is determined by `char`, so this cannot make a character
    /// wider than `char.x`.
    #[serde(default)]
    pub width_overrides: HashMap<char, u32>,
}

impl Metrics {
    /// The padded width of one character in the font.
    #[must_use]
    pub fn padded_w(&self) -> u32 {
        self.char.w + self.pad.w
    }

    /// The padded height of one character in the font.
    #[must_use]
    pub fn padded_h(&self) -> u32 {
        self.char.h + self.pad.h
    }

    /// Signed maximal size of a horizontal span `size` characters wide.
    ///
    /// This is the result of multiplying `size` by the padded baseline width
    /// of the font, ignoring any kerning or proportionality adjustments.
    /// This is useful for aligning items on a character grid but may
    /// overestimate widths on proportional fonts.
    ///
    /// If `size` is negative, the result will be negative.
    #[must_use]
    pub fn span_w(&self, size: i32) -> i32 {
        sat_i32(self.padded_w()) * size
    }

    /// Like `span_w`, but accurately calculates the width of `str`.
    ///
    /// This performs the same positioning calculations as text rendering, and
    /// is accurate in the face of any proportionality in the font.
    #[must_use]
    pub fn span_w_str(&self, str: &str) -> i32 {
        // Pretend to lay out the string, then work out where the last character went.
        self.layout_str(Point::default(), str)
            .last()
            .map_or(0, |glyph| {
                glyph.dst.x(0, super::super::metrics::anchor::X::Right)
            })
    }

    /// Signed maximal size of a vertical span `size` characters tall.
    ///
    /// This is the result of multiplying `size` by the padded baseline height
    /// of the font.
    ///
    /// If `size` is negative, the result will be negative.
    #[must_use]
    pub fn span_h(&self, size: i32) -> i32 {
        sat_i32(self.padded_h()) * size
    }

    /// Converts a size in chars into a size in pixels.
    #[must_use]
    pub fn text_size(&self, w_chars: i32, h_chars: i32) -> Size {
        Size::from_i32s(self.span_w(w_chars), self.span_h(h_chars))
    }

    /// Calculates layout for a byte-string as a series of [Glyph]s.
    pub fn layout_str<'a, B: AsRef<[u8]> + ?Sized>(
        &'a self,
        start: Point,
        bytes: &'a B,
    ) -> impl Iterator<Item = Glyph> + 'a {
        bytes.as_ref().iter().scan(start, move |point, char| {
            // TODO(@MattWindsor91): proportionality
            let src = self.glyph_rect(*char);
            let offset = sat_i32(src.size.w) + sat_i32(self.pad.w);
            let next_point = point.offset(offset, 0);
            let dst_tl = std::mem::replace(point, next_point);
            let dst = Rect {
                top_left: dst_tl,
                ..src
            };
            Some(Glyph { src, dst })
        })
    }

    /// Bounding box for a glyph in the texture.
    #[must_use]
    fn glyph_rect(&self, char: u8) -> Rect {
        self.glyph_top_left(char).to_rect(self.glyph_size(char))
    }

    /// The top-left position of the glyph for `char` in the font.
    #[must_use]
    fn glyph_top_left(&self, char: u8) -> Point {
        Point {
            x: glyph_axis(glyph_col(char), self.padded_w()),
            y: glyph_axis(glyph_row(char), self.padded_h()),
        }
    }

    /// The size of the glyph for `char` in the font.
    #[must_use]
    fn glyph_size(&self, char: u8) -> Size {
        let mut size = self.char;

        if let Some(w) = self.glyph_override(char) {
            size.w = w;
        }

        size
    }

    fn glyph_override(&self, char: u8) -> Option<u32> {
        char::from_u32(char.into())
            .and_then(|x| self.width_overrides.get(&x))
            .copied()
    }
}

/// Calculates one axis of the top-left of the glyph.
#[must_use]
fn glyph_axis(index: u8, size: u32) -> i32 {
    // Can't multiply _then_ convert, because of overflow on big fonts.
    i32::from(index) * sat_i32(size)
}

/// A representation of a glyph to be rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Glyph {
    /// The glyph's location inside the texture map.
    pub src: Rect,
    /// Where to render the glyph.
    pub dst: Rect,
}

/// The column of a glyph in the font.
#[must_use]
pub fn glyph_col(char: u8) -> u8 {
    char % NUM_COLS
}

/// The row of a glyph in the font.
#[must_use]
pub fn glyph_row(char: u8) -> u8 {
    char / NUM_COLS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big_font() -> Metrics {
        Metrics {
            char: Size { w: 9, h: 9 },
            pad: Size { w: 1, h: 1 },
            width_overrides: HashMap::new(),
        }
    }

    /// Tests that the X co-ordinate of `glyph_top_left` works correctly without
    /// overflow on a big bitmap.
    #[test]
    fn glyph_x_overflow() {
        assert_eq!(big_font().glyph_top_left(31).x, 310);
    }

    /// Tests that the Y co-ordinate of `glyph_top_left` works correctly without
    /// overflow on a big bitmap.
    #[test]
    fn glyph_y_overflow() {
        assert_eq!(big_font().glyph_top_left(255).y, 70);
    }
}
