//! The zombiesplit library top-level.
#![warn(clippy::all, clippy::pedantic)]

pub mod config;
pub mod model;
pub mod presenter;
pub mod view;

pub use presenter::Presenter;
pub use view::View;
