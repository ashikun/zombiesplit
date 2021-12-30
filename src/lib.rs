//! The zombiesplit library top-level.
#![warn(clippy::all, clippy::pedantic)]

pub mod cli;
pub mod config;
pub mod db;
pub mod model;
pub mod server;
pub mod ui;
pub mod zombie;

pub use db::Db;
