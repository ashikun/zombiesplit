//! Font metrics.
use serde::{Deserialize, Serialize};

/// A font metrics set.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Metrics {
    /// Columns in the font texture.
    /// The number of rows is 256 divided by the number of columns.
    pub cols: u8,
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

    /// The column of a glyph in the font.
    #[must_use]
    pub fn glyph_col(self, char: u8) -> u8 {
        char % self.cols
    }

    /// The row of a glyph in the font.
    #[must_use]
    pub fn glyph_row(self, char: u8) -> u8 {
        char / self.cols
    }

    /// The left position of a glyph in the font.
    #[must_use]
    pub fn glyph_x(self, char: u8) -> i32 {
        // Can't multiply _then_ convert, because of overflow on big fonts.
        i32::from(self.glyph_col(char)) * i32::from(self.padded_w())
    }

    /// The top position of a glyph in the font.
    #[must_use]
    pub fn glyph_y(self, char: u8) -> i32 {
        // Can't multiply _then_ convert, because of overflow on big fonts.
        i32::from(self.glyph_row(char)) * i32::from(self.padded_h())
    }

    /// The size of a horizontal padded character span.
    #[must_use]
    pub fn span_w(self, size: i32) -> i32 {
        i32::from(self.padded_w()) * size
    }

    /// The size of a vertical padded character span.
    #[must_use]
    pub fn span_h(self, size: i32) -> i32 {
        i32::from(self.padded_h()) * size
    }
}

/// Trait for things that carry font metrics.
pub trait Source<Key> {
    /// Gets the given font's metrics set.
    #[must_use]
    fn metrics(&self, id: Key) -> Metrics;
}

#[cfg(test)]
mod tests {
    use super::*;

    const BIG_FONT: Metrics = Metrics {
        cols: 32,
        char: Pair { w: 9, h: 9 },
        pad: Pair { w: 1, h: 1 },
    };

    /// Tests that `glyph_x` works correctly without overflow on a big bitmap.
    #[test]
    fn glyph_x_overflow() {
        assert_eq!(BIG_FONT.glyph_x(31), 310)
    }

    /// Tests that `glyph_y` works correctly without overflow on a big bitmap.
    #[test]
    fn glyph_y_overflow() {
        assert_eq!(BIG_FONT.glyph_y(255), 70)
    }
}
