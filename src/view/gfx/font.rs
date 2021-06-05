//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

pub mod error;
pub mod manager;
pub mod metrics;
pub mod set;
pub use error::{Error, Result};
pub use manager::Manager;
pub use metrics::Metrics;
pub use set::{Config, Id, Set};
