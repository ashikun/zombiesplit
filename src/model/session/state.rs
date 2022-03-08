/*! Session state.
 *
 * This is mainly exported outside of the session for the purposes of dumping a
 */

use super::{
    super::super::model::{short, timing},
    split, Run,
};

/// The state of a session.
///
/// The session state contains both a run and its current comparison.
/// A session's state can be dumped out at any point.
#[derive(Clone, Debug)]
pub struct State {
    pub run: Run,
    /// Comparison data for the game/category currently being run.
    pub comparison: timing::Comparison,
}

impl State {
    /// Gets a mutable reference to the split at the given location.
    #[must_use]
    pub fn get_split_mut(&mut self, split: impl split::Locator) -> Option<&mut split::Split> {
        self.run.splits.get_mut(split)
    }

    /// Tries to locate the given split and, if found, pushes the given time to it.
    ///
    /// Returns the short-name of the split if successful.
    pub fn push_to(
        &mut self,
        split: impl split::Locator,
        time: timing::Time,
    ) -> Option<short::Name> {
        self.get_split_mut(split).map(|s| {
            s.push(time);
            s.info.short
        })
    }

    /// Tries to locate the given split and, if found, pops the most recent time from it.
    ///
    /// Returns the short-name of the split and popped time if fully successful.
    pub fn pop_from(&mut self, split: impl split::Locator) -> Option<(short::Name, timing::Time)> {
        self.get_split_mut(split).and_then(|s| {
            let short = s.info.short;
            s.pop().map(|time| (short, time))
        })
    }

    /// Tries to locate the given split and, if found, clears all times from it.
    ///
    /// Returns the short-name of the split if successful.
    pub fn clear_at(&mut self, split: impl split::Locator) -> Option<short::Name> {
        self.get_split_mut(split).map(|s| {
            s.clear();
            s.info.short
        })
    }
}
