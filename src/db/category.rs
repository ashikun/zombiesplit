//! Module for database activities related to storing and querying categories.

pub mod get;
pub mod id;

pub use get::Getter;
pub use id::{GcID, Locator};
