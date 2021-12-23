//! Conversion functions from zombiesplit metrics to SDL ones.

use super::super::view::gfx::metrics;

/// Converts a rect from zombiesplit to SDL.
#[must_use]
pub fn convert_rect(r: &metrics::Rect) -> sdl2::rect::Rect {
    let w = u32_or_zero(r.size.w);
    let h = u32_or_zero(r.size.h);
    sdl2::rect::Rect::new(r.top_left.x, r.top_left.y, w, h)
}

/// Convert `x` to u32, set to 0 if negative.
pub(crate) fn u32_or_zero(x: impl TryInto<u32>) -> u32 {
    x.try_into().unwrap_or_default()
}
