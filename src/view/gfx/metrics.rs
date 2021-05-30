//! Font and window metrics (most of which will be un-hardcoded later).
use std::convert::TryFrom;

pub(super) const TIME_CHARS: i32 = 9; // XX'XX"XXX

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
    /// The horizontal padding on header contents.
    pub header_xpad: u32,
    /// The vertical padding on header contents.
    pub header_ypad: u32,
    /// The horizontal padding on split names and times.
    pub split_xpad: u32,
    /// The vertical position where the split times start.
    pub split_ypos: u32,
    /// The height of a split, including any padding between it and the next split.
    pub split_h: u32,
}

/// Hardcoded metrics for the window in zombiesplit (for now).
pub const WINDOW: Window = Window {
    win_w: 320,
    win_h: 640,
    header_xpad: 8,
    header_ypad: 8,
    split_xpad: 4,
    split_h: 16,
    split_ypos: 48,
};

impl Window {
    /// Gets the bounding box of the header part of the window.
    #[must_use]
    pub fn header_rect(&self) -> Rect {
        Rect {
            x: sat_i32(self.header_xpad),
            y: sat_i32(self.header_ypad),
            w: self.win_w - (2 * self.header_xpad),
            h: self.split_ypos - (2 * self.header_ypad),
        }
    }

    /// Gets the bounding box of the splits part of the window.
    #[must_use]
    pub fn splits_rect(&self) -> Rect {
        Rect {
            x: sat_i32(self.split_xpad),
            y: sat_i32(self.split_ypos),
            w: self.win_w - (2 * self.split_xpad),
            h: self.win_h - self.split_ypos,
        }
    }
}

/// Convert `x` to i32, saturate if overly long.
pub(crate) fn sat_i32<T>(x: T) -> i32
where
    i32: TryFrom<T>,
{
    i32::try_from(x).unwrap_or(i32::MAX)
}

/// Output-independent rectangle.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// X position of left.
    pub x: i32,
    /// Y position of top.
    pub y: i32,
    /// Width in pixels.
    pub w: u32,
    /// Height in pixels.
    pub h: u32,
}

impl Rect {
    /// Gets the right X co-ordinate.
    #[must_use]
    pub fn x2(self) -> i32 {
        self.x + sat_i32(self.w)
    }

    /// Gets the bottom Y co-ordinate.
    #[must_use]
    pub fn y2(self) -> i32 {
        self.y + sat_i32(self.h)
    }
}
