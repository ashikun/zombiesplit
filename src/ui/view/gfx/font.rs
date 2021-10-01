//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

pub mod error;
pub mod map;
pub mod metrics;
pub use error::{Error, Result};
pub use map::{Config, Id, Map, Spec};
pub use metrics::Metrics;
