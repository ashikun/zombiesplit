/*! Events and storage for sending information about aggregate times on splits.

zombiesplit tracks two [Scope]s of aggregate (total across all times in the
split, and cumulative totals over all splits up to and including the split),
and two [Source]s (from the current attempt, and from the database comparison).

This module contains both the event building blocks used to transmit
observations of aggregate changes from the module, as well as structures for
storing those aggregates.
*/

pub mod index;
pub mod set;

pub use index::{Kind, Scope, Source};
pub use set::{Full, Set};
