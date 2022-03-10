/*! Session state.
 *
 * This is mainly exported outside of the session for the purposes of dumping the state to clients
 * as they connect to the server.
 */

use super::{
    super::super::model::{short, timing},
    split, Attempt,
};
use std::collections::HashMap;

/// The state of a session.
///
/// The session state contains both a run and its current comparison.
/// A session's state can be dumped out at any point.
#[derive(Clone, Debug)]
pub struct State {
    /// The current attempt.
    pub run: Attempt,
    /// Comparison data for the game/category currently being run.
    pub comparison: timing::Comparison,
    /// Pre-cached extra data for the splits.
    ///
    /// This is kept separate from the attempt itself as it is effectively a denormalised
    /// derivative of the attempt data.
    pub notes: HashMap<short::Name, SplitNote>,
    /// Total for the run.
    pub total: Option<timing::comparison::PacedTime>,
}

impl State {
    /// Constructs a new run with the given attempt and comparison.
    ///
    /// Split notes will be repopulated from scratch.
    #[must_use]
    pub fn new(run: Attempt, comparison: timing::Comparison) -> Self {
        let mut result = Self {
            run,
            comparison,
            notes: std::collections::HashMap::default(),
            total: None,
        };
        result.recalculate_indirect_fields();
        result
    }

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
        self.act_on_split(split, |s| s.push(time))
    }

    /// Tries to locate the given split and, if found, pops the most recent time from it.
    ///
    /// Returns the short-name of the split if fully successful.
    pub fn pop_from(&mut self, split: impl split::Locator) -> Option<short::Name> {
        self.act_on_split(split, |s| {
            let _ = s.pop();
        })
    }

    /// Tries to locate the given split and, if found, clears all times from it.
    ///
    /// Returns the short-name of the split if successful.
    pub fn clear_at(&mut self, split: impl split::Locator) -> Option<short::Name> {
        self.act_on_split(split, split::Split::clear)
    }

    /// Common pattern of various actions on splits.
    fn act_on_split(
        &mut self,
        split: impl split::Locator,
        f: impl FnOnce(&mut split::Split),
    ) -> Option<short::Name> {
        self.get_split_mut(split)
            .map(|s| {
                f(s);
                s.info.short
            })
            .map(|result| {
                self.recalculate_indirect_fields();
                result
            })
    }

    fn recalculate_indirect_fields(&mut self) {
        self.recalculate_notes();
        self.recalculate_total();
    }

    fn recalculate_notes(&mut self) {
        // TODO(@MattWindsor91): only recalculate from the changed split.
        self.notes = self
            .run
            .splits
            .aggregates()
            .map(|(s, a)| (s.info.short, self.note(s, a)))
            .collect();
    }

    fn note(&self, split: &split::Split, aggregates: timing::aggregate::Set) -> SplitNote {
        SplitNote {
            aggregates,
            pace: self.split_pace(split, aggregates),
        }
    }

    fn split_pace(
        &self,
        split: &split::Split,
        aggregates: timing::aggregate::Set,
    ) -> timing::comparison::pace::SplitInRun {
        if split.num_times() == 0 {
            timing::comparison::pace::SplitInRun::Inconclusive
        } else {
            self.comparison.pace(split.info.short, aggregates)
        }
    }

    fn recalculate_total(&mut self) {
        self.total = self
            .notes
            .iter()
            .max_by_key(|(_, note)| note.aggregates.cumulative)
            .map(|(_, n)| n.paced_cumulative());
    }
}

/// A precomputed series of facts about a split.
///
/// These are things that the client could compute itself, but which we keep in the state to
/// centralise computation and allow for simpler client logic.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SplitNote {
    /// Attempt-level aggregates for this split.
    ///
    /// Comparison-level aggregates are in the comparison.
    pub aggregates: timing::aggregate::Set,
    /// Pace note for this split.
    pub pace: timing::comparison::pace::SplitInRun,
}

impl SplitNote {
    /// Extracts the cumulative of this note, alongside its overall-run pace.
    #[must_use]
    pub fn paced_cumulative(&self) -> timing::comparison::PacedTime {
        timing::comparison::PacedTime {
            pace: self.pace.overall(),
            time: self.aggregates.cumulative,
        }
    }
}
