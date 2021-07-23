//! The zombiesplit library top-level.
#![warn(clippy::all, clippy::pedantic)]

pub mod config;
pub mod db;
pub mod model;
pub mod ui;
pub mod zombie;

pub use db::Db;
pub use zombie::Zombie;
