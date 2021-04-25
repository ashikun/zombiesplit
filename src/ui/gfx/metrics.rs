//! Font metrics.
use sdl2::rect::{Point, Rect};

/// Number of columns in the font bitmap.
/// The number of rows is 256 divided by the number of columns.
const COLS: u8 = 32;

/// Width of one character in the font, without padding.
const W: u8 = 7;

/// Height of one character in the font, without padding.
const H: u8 = 9;

/// Width of one character in the font, plus padding.
const WPAD: i32 = (W as i32) + 1;
/// Height of one character in the font, plus padding.
const HPAD: i32 = (H as i32) + 1;

/// Produces a rectangle with top-left `top_left` and the size of one font
/// character.
#[must_use]
pub fn char_rect(top_left: Point) -> Rect {
    Rect::new(top_left.x, top_left.y, u32::from(W), u32::from(H))
}

/// Produces the appropriate rectangle for looking up `char` in the font.
#[must_use]
pub fn font_rect(char: u8) -> Rect {
    let col = i32::from(char % COLS);
    let row = i32::from(char / COLS);
    char_rect(Point::new(col * WPAD, row * HPAD))
}

/// Offsets `point` by `dx` padded characters horizontally and `dy` vertically.
#[must_use]
pub fn offset(point: Point, dx: i32, dy: i32) -> Point {
    point.offset(dx * WPAD, dy * HPAD)
}
