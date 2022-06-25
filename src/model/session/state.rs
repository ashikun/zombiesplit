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
    pub attempt: Attempt,
    /// Comparison data for the game/category currently being run.
    pub comparison: timing::Comparison,
    /// Pre-cached extra data for the splits.
    ///
    /// This is kept separate from the attempt itself as it is effectively a denormalised
    /// derivative of the attempt data.
    pub notes: HashMap<short::Name, SplitNote>,
    /// Total for the run, including a delta against its comparison.
    pub total: Option<timing::comparison::delta::Time>,
}

impl State {
    /// Constructs a new run with the given attempt and comparison.
    ///
    /// Split notes will be repopulated from scratch.
    #[must_use]
    pub fn new(run: Attempt, comparison: timing::Comparison) -> Self {
        let mut result = Self {
            attempt: run,
            comparison,
            notes: HashMap::default(),
            total: None,
        };
        result.reset_notes();
        result
    }

    /// Resets the state of the run.
    ///
    /// The result of applying this should be equivalent to producing a new state.
    pub fn reset(&mut self, dest: super::action::OldDestination) {
        self.attempt.reset(dest);
        self.reset_notes();
        self.total = None;
    }

    /// Gets a mutable reference to the split at the given location.
    #[must_use]
    fn get_split_mut(&mut self, split: impl split::Locator) -> Option<&mut split::Split> {
        self.attempt.splits.get_mut(split)
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

    /// Populates the notes table with an empty note for each split.
    fn reset_notes(&mut self) {
        for s in self.attempt.splits.iter() {
            self.notes.insert(s.info.short, SplitNote::default());
        }
    }

    /// Populates the notes table with recalculated notes.
    fn recalculate_notes(&mut self) {
        // TODO(@MattWindsor91): only recalculate from the changed split.
        for (s, a) in self.attempt.splits.aggregates() {
            self.notes.insert(s.info.short, self.note(s, a));
        }
    }

    fn note(&self, split: &split::Split, aggregates: timing::aggregate::Set) -> SplitNote {
        SplitNote {
            aggregates,
            delta: self.split_delta(split, aggregates),
        }
    }

    fn split_delta(
        &self,
        split: &split::Split,
        aggregates: timing::aggregate::Set,
    ) -> Option<timing::comparison::delta::Split> {
        if split.num_times() == 0 {
            None
        } else {
            // TODO: maybe propagate the error here?
            self.comparison.delta(split.info.short, aggregates).ok()
        }
    }

    fn recalculate_total(&mut self) {
        self.total = self
            .attempt
            .splits
            .last_entered()
            .and_then(|s| self.notes.get(&s.info.short))
            .and_then(|note| {
                note.delta.map(|d| super::comparison::delta::Time {
                    time: note.aggregates.cumulative,
                    delta: d.run(),
                })
            });
    }
}

/// A precomputed series of facts about a split.
///
/// These are things that the client could compute itself, but which we keep in the state to
/// centralise computation and allow for simpler client logic.
#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub struct SplitNote {
    /// Attempt-level aggregates for this split.
    ///
    /// Comparison-level aggregates are in the comparison.
    pub aggregates: timing::aggregate::Set,
    /// Delta between this split and comparison.
    /// May be missing, if there are no times.
    pub delta: Option<timing::comparison::delta::Split>,
}

impl SplitNote {
    /// Extracts the cumulative time of this note, alongside its delta against the comparison.
    ///
    /// If there have been no splits yet, this is `None`.
    #[must_use]
    pub fn paced_cumulative(&self) -> Option<timing::comparison::PacedTime> {
        // Assuming that self.delta == None <-> no splits, and we don't need to capture this
        // separately in the aggregates.
        self.delta.map(|d| timing::comparison::PacedTime {
            pace: d.pace.overall(),
            time: self.aggregates.cumulative,
        })
    }
}
