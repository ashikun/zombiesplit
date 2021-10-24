//! Font management services.

// TODO(@MattWindsor91): decouple SDL here?

pub use error::{Error, Result};
pub use map::{Id, Map, Spec};
pub use metrics::Metrics;

pub mod error;
pub mod map;
pub mod metrics;
