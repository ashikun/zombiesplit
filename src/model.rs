//! Models used in zombiesplit.
pub mod aggregate;
pub mod attempt;
pub mod comparison;
pub mod game;
pub mod history;
pub mod load;
pub mod short;
pub mod time;

pub use self::time::Time;
pub use load::Loadable;
