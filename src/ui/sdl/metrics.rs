//! Conversion functions from zombiesplit metrics to SDL ones.

use super::super::view::gfx::metrics;

/// Converts a rect from zombiesplit to SDL.
#[must_use]
pub fn convert_rect(r: &metrics::Rect) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(r.top_left.x, r.top_left.y, r.size.w, r.size.h)
}
