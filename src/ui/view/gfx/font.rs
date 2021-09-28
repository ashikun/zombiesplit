//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

pub mod error;
pub mod metrics;
pub mod set;
pub use error::{Error, Result};
pub use metrics::Metrics;
pub use set::{Config, Id, Set, Spec};
