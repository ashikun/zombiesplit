//! Font metrics.
use super::super::metrics::{conv::sat_i32, Point, Rect, Size};
use serde::{Deserialize, Serialize};

/// The number of columns in a font.
const NUM_COLS: u8 = 32;

/// A font metrics set.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Metrics {
    /// Dimensions of one character in the font, without padding.
    pub char: Pair,
    /// Dimensions of padding between characters in the font.
    pub pad: Pair,
}

/// A width-height pair.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Pair {
    /// Width of a font element, in pixels.
    pub w: u8,
    /// Height of a font element, in pixels.
    pub h: u8,
}

impl Metrics {
    /// The padded width of one character in the font.
    #[must_use]
    pub fn padded_w(self) -> u8 {
        self.char.w + self.pad.w
    }

    /// The padded height of one character in the font.
    #[must_use]
    pub fn padded_h(self) -> u8 {
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
        i32::from(self.padded_w()) * size
    }

    /// Like `span_w`, but accurately calculates the width of `str`.
    ///
    /// This performs the same positioning calculations as text rendering, and
    /// is accurate in the face of any proportionality in the font.
    #[must_use]
    pub fn span_w_str(&self, str: &str) -> i32 {
        // If we ever implement proportional fonts, this'll change.
        // I'd recommend making a general positioning algorithm and then using
        // it for both span_w_str and rendering.
        self.span_w(sat_i32(str.len()))
    }

    /// Signed maximal size of a vertical span `size` characters tall.
    ///
    /// This is the result of multiplying `size` by the padded baseline height
    /// of the font.
    ///
    /// If `size` is negative, the result will be negative.
    #[must_use]
    pub fn span_h(&self, size: i32) -> i32 {
        i32::from(self.padded_h()) * size
    }

    /// Converts a size in chars into a size in pixels.
    #[must_use]
    pub fn text_size(&self, w_chars: i32, h_chars: i32) -> Size {
        Size::from_i32s(self.span_w(w_chars), self.span_h(h_chars))
    }

    /// Calculates layout for a string as a series of [Glyph]s.
    pub fn layout_str<'a>(
        &'a self,
        start: Point,
        str: &'a str,
    ) -> impl Iterator<Item = Glyph> + 'a {
        str.as_bytes().iter().scan(start, move |point, char| {
            // TODO(@MattWindsor91): proportionality
            let src = self.glyph_rect(*char);
            let next_point = point.offset(self.span_w(1), 0);
            let dst_tl = std::mem::replace(point, next_point);
            let dst = Rect {
                top_left: dst_tl,
                ..src
            };
            Some(Glyph { src, dst })
        })
    }

    /// Bounding box for a glyph in the texture.
    fn glyph_rect(self, char: u8) -> Rect {
        Rect::new(
            self.glyph_x(char),
            self.glyph_y(char),
            self.char.w.into(),
            self.char.h.into(),
        )
    }

    /// The left position of a glyph in the font.
    #[must_use]
    fn glyph_x(self, char: u8) -> i32 {
        // TODO(@MattWindsor91): stop conflating visible padding with charmap padding
        // Can't multiply _then_ convert, because of overflow on big fonts.
        i32::from(glyph_col(char)) * i32::from(self.padded_w())
    }

    /// The top position of a glyph in the font.
    #[must_use]
    fn glyph_y(self, char: u8) -> i32 {
        // TODO(@MattWindsor91): stop conflating visible padding with charmap padding
        // Can't multiply _then_ convert, because of overflow on big fonts.
        i32::from(glyph_row(char)) * i32::from(self.padded_h())
    }
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

    const BIG_FONT: Metrics = Metrics {
        char: Pair { w: 9, h: 9 },
        pad: Pair { w: 1, h: 1 },
    };

    /// Tests that `glyph_x` works correctly without overflow on a big bitmap.
    #[test]
    fn glyph_x_overflow() {
        assert_eq!(BIG_FONT.glyph_x(31), 310);
    }

    /// Tests that `glyph_y` works correctly without overflow on a big bitmap.
    #[test]
    fn glyph_y_overflow() {
        assert_eq!(BIG_FONT.glyph_y(255), 70);
    }
}
