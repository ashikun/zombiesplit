//! Models used in zombiesplit.
pub mod attempt;
pub mod comparison;
pub mod game;
pub mod session;
pub mod short;
pub mod time;

pub use session::{Metadata, Session};
pub use time::Time;
