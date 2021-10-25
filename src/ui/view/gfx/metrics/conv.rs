//! Conversion helpers.

/// Convert `x` to i32, saturate if overly long.
pub(crate) fn sat_i32(x: impl TryInto<i32>) -> i32 {
    x.try_into().unwrap_or(i32::MAX)
}

/// Convert `x` to u32, set to 0 if negative.
pub(crate) fn u32_or_zero(x: impl TryInto<u32>) -> u32 {
    x.try_into().unwrap_or_default()
}
