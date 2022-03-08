//! Editor state,

use crate::model::timing::time;
use std::ops::{Index, IndexMut};

/// Presenter state about an editor.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Editor {
    /// The current field being edited, if any.
    pub field: Option<time::Position>,
    /// The current hours string.
    ///
    /// This is not actually exposed anywhere yet, but exists to simplify the index impl.
    pub hours: String,
    /// The current minutes string.
    pub mins: String,
    /// The current seconds string.
    pub secs: String,
    /// The current milliseconds string.
    pub msecs: String,
}

impl Editor {
    /// Gets a readout of the field at position `pos`.
    #[must_use]
    pub fn field(&self, pos: time::Position) -> &str {
        &*self[pos]
    }
}

impl Index<time::Position> for Editor {
    type Output = String;

    // TODO(@MattWindsor91): this returns &String, which is somewhat odd.
    fn index(&self, index: time::Position) -> &Self::Output {
        match index {
            time::Position::Hours => &self.hours,
            time::Position::Minutes => &self.mins,
            time::Position::Seconds => &self.secs,
            time::Position::Milliseconds => &self.msecs,
        }
    }
}

impl IndexMut<time::Position> for Editor {
    fn index_mut(&mut self, index: time::Position) -> &mut Self::Output {
        match index {
            time::Position::Hours => &mut self.hours,
            time::Position::Minutes => &mut self.mins,
            time::Position::Seconds => &mut self.secs,
            time::Position::Milliseconds => &mut self.msecs,
        }
    }
}
