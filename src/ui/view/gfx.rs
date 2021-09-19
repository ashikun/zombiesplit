/*! Graphics rendering code for zombiesplit.

This module contains the (semi-)low-level code for displaying the zombiesplit
(reference) user interface using SDL.  It handles colour and font look-up,
metrics, rendering,  and so on.
*/

pub mod colour;
pub mod font;
pub mod metrics; // for now
pub mod pen;
pub mod position;
pub mod render;

use super::error::{Error, Result};

/// Makes a zombiesplit window.
///
/// # Errors
///
/// Returns an error if SDL fails to make the window.
pub fn make_window(
    video: &sdl2::VideoSubsystem,
    wmetrics: metrics::Window,
) -> Result<sdl2::video::Window> {
    let window = video
        .window("zombiesplit", wmetrics.win_w, wmetrics.win_h)
        .position_centered()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
