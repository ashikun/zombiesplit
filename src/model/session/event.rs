//! Observer pattern wiring for attempt sessions.

pub mod debug;
pub mod mux;
pub mod observer;
pub mod split;
pub mod time;

use super::super::{game::category, short, timing};
use serde::{Deserialize, Serialize};

pub use debug::Debug;
pub use mux::Mux;
pub use observer::{Observable, Observer};
pub use split::Split;
pub use time::Time;

/// Enumeration of events that can be sent through an observer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    /// Observes a change in one of the run total times.
    Total(Total, Option<timing::Time>),
    /// Observes information about a reset, with the new attempt information attached.
    Reset(category::AttemptInfo),
    /// Observes an event on a split.
    Split(short::Name, Split),
}

/// Information about a type of total.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Total {
    /// The total is the attempt total, and has the given delta from the comparison total.
    Attempt(timing::comparison::Delta),
    /// The total is one of the comparison totals.
    Comparison(timing::comparison::run::TotalType),
}
