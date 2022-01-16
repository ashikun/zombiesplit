/*! The zombiesplit timing model.

This consists of low-level model components that track times, aggregates of times, and comparisons
between current-run times and historic-run times. */
pub mod aggregate;
pub mod comparison;
pub mod time;

pub use comparison::Comparison;
pub use time::Time;
