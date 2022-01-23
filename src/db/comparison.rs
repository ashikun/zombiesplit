/*! Database queries related to comparison data.

Comparison data can't be inserted directly into the database.  Instead, it comes through views
indirectly pulling out run and category data.
*/

pub mod get;
mod sql;

pub use get::Getter;
