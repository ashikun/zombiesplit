//! Font and window metrics (most of which will be un-hardcoded later).
use std::convert::TryFrom;

const TIME_CHARS: i32 = 9; // XX'XX"XXX

/// Font metrics.
#[derive(Copy, Clone)]
pub struct Font {
    /// Columns in the font texture.
    /// The number of rows is 256 divided by the number of columns.
    pub cols: u8,
    /// Width of one character in the font, without padding.
    pub char_w: u8,
    /// Height of one character in the font, without padding.
    pub char_h: u8,
    /// Horizontal padding between characters in the font.
    pub pad_w: u8,
    /// Vertical padding between characters in the font.
    pub pad_h: u8,
}

/// Hardcoded metrics for the one font in zombiesplit (for now).
pub const FONT: Font = Font {
    cols: 32,
    char_w: 7,
    char_h: 9,
    pad_w: 1,
    pad_h: 1,
};

impl Font {
    /// The padded width of one character in the font.
    #[must_use]
    pub fn padded_w(self) -> u8 {
        self.char_w + self.pad_w
    }

    /// The padded height of one character in the font.
    #[must_use]
    pub fn padded_h(self) -> u8 {
        self.char_h + self.pad_h
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
        i32::from(self.glyph_col(char) * self.padded_w())
    }

    /// The top position of a glyph in the font.
    #[must_use]
    pub fn glyph_y(self, char: u8) -> i32 {
        i32::from(self.glyph_row(char) * self.padded_h())
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

/// Window metrics.
#[derive(Copy, Clone)]
pub struct Window {
    /// The window width.
    pub win_w: u32,
    /// The window height.
    pub win_h: u32,
    /// The horizontal padding on split names and times.
    pub split_xpad: i32,
    /// The vertical position where the split times start.
    pub split_ypos: i32,
    /// The height of a split, including any padding between it and the next split.
    pub split_h: i32,
}

/// Hardcoded metrics for the window in zombiesplit (for now).
pub const WINDOW: Window = Window {
    win_w: 320,
    win_h: 640,
    split_xpad: 4,
    split_h: 16,
    split_ypos: 4,
};

impl Window {
    /// Gets the left position for the split time, given font metrics.
    #[must_use]
    pub fn split_time_x(&self, font: Font) -> i32 {
        // TODO(@MattWindsor91): take font metrics.
        sat_i32(self.win_w) - (self.split_xpad + font.span_w(TIME_CHARS))
    }

    /// Gets the Y position of the given split.
    #[must_use]
    pub fn split_y(&self, num: usize) -> i32 {
        self.split_ypos + (self.split_h * sat_i32(num))
    }
}

/// Convert `x` to i32, saturate if overly long.
fn sat_i32<T>(x: T) -> i32
where
    i32: TryFrom<T>,
{
    i32::try_from(x).unwrap_or(i32::MAX)
}
