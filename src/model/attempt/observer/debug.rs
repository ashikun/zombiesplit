//! A logging debug observer.

use log::debug;

use super::Observer;

/// A debug observer.
///
/// Every time an observation occurs, this observer logs it as a debug message.
pub struct Debug;

impl Observer for Debug {
    fn observe(&self, evt: super::Event) {
        debug!("observed {evt:?}");
    }
}
