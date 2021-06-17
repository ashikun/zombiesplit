//! The zombiesplit library top-level.
#![warn(clippy::all, clippy::pedantic)]

pub mod config;
pub mod db;
pub mod model;
pub mod presenter;
pub mod view;
pub mod zombie;

pub use db::Db;
pub use presenter::Presenter;
pub use view::View;
pub use zombie::Zombie;
