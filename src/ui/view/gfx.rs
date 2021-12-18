/*! Low-ish graphics primitives and traits used by the zombiesplit UI.

This module contains traits used for low-level rendering (implemented by UI
backends such as SDL), as well as concepts such as colours, fonts, and metrics.
*/

pub use error::{Error, Result};
pub use render::Renderer;
pub use writer::Writer;

pub mod colour;
pub mod error;
pub mod font;
pub mod metrics; // for now
pub mod render;
pub mod writer;
