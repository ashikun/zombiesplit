/*! The presenter's internal state.

This is populated from the model every time the presenter observes some kind
of change on the model.
*/

use crate::model::{
    attempt::observer::{aggregate, split},
    comparison::pace,
    game::category,
    short,
};

/// The presenter's representation of the model.
#[derive(Debug, Default)]
pub struct State {
    /// The current attempt information.
    pub attempt: category::AttemptInfo,
    /// Information about the game and category being played.
    pub game_category: category::Info,

    // TODO(@MattWindsor91): this has a lot of invariants that need to be
    // maintained, so we should hide these next two fields or move the
    // short-position map elsewhere.
    /// Bidirectional map from split shortnames to their locations on the UI.
    pub short_map: short::Bimap<usize>,
    /// Split states.
    pub splits: Vec<Split>, // TODO(@MattWindsor91): move things from using Session to using this.
}

impl State {
    /// Makes the presenter state reflect a reset in the run.
    ///
    /// This clears the time count aggregate data for all splits.  It doesn't
    /// change the attempt information, as we expect there will be a separate
    /// observation for that.
    pub fn reset(&mut self) {
        for split in &mut self.splits {
            split.reset();
        }
    }

    /// Inserts a split into the split list with the given display name.
    pub fn add_split(&mut self, short: String, name: String) {
        let split = Split {
            name,
            ..Split::default()
        };
        self.splits.push(split);
        self.short_map.insert(short, self.splits.len() - 1);
    }

    /// Handles an observation for the split with the given shortname.
    pub fn handle_split_event(&mut self, short: &str, evt: split::Event) {
        if let Some(ref mut s) = self
            .short_map
            .get_by_left(short)
            .copied()
            .and_then(|x| self.splits.get_mut(x))
        {
            s.handle_event(evt);
            // TODO(@MattWindsor91): open editor
        }
    }

    /// Gets the number of splits that the presenter is tracking.
    ///
    /// ```
    /// use zombiesplit::ui::presenter::state;
    ///
    /// let mut s = state::State::default();
    /// assert_eq!(0, s.num_splits());
    ///
    /// s.add_split("pp1".to_owned(), "Palmtree Panic 1".to_owned());
    /// s.add_split("sp1".to_owned(), "Special Stage 1".to_owned());
    /// s.add_split("pp2".to_owned(), "Palmtree Panic 2".to_owned());
    /// assert_eq!(3, s.num_splits());
    /// ```
    #[must_use]
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }
}

/// Presenter state about one split.
#[derive(Debug, Default, Clone)]
pub struct Split {
    /// The number of times that have been logged on this split.
    pub num_times: usize,
    /// The display name of the split.
    pub name: String,
    /// The aggregate times logged for this split.
    pub aggregates: aggregate::Set,
    /// The pace of this split in the run-so-far.
    pub pace_in_run: pace::SplitInRun,
}

impl Split {
    /// Resets the per-run state of this split.
    ///
    /// This clears the aggregates, pacing information, and time count; it
    /// doesn't reset metadata.
    pub fn reset(&mut self) {
        self.num_times = 0;
        self.aggregates = aggregate::Set::default();
        self.pace_in_run = pace::SplitInRun::default();
    }

    /// Handles an observation for this split.
    pub fn handle_event(&mut self, evt: split::Event) {
        match evt {
            split::Event::Time(time, split::Time::Aggregate(kind)) => {
                self.aggregates[kind.source][kind.scope] = Some(time);
            }
            split::Event::Time(_, split::Time::Pushed) => {
                self.num_times += 1;
            }
            split::Event::Time(_, split::Time::Popped) => {
                self.num_times -= 1;
                // Moving the newly popped time to the editor gets handled
                // elsewhere.
            }
            split::Event::Pace(pace) => {
                self.pace_in_run = pace;
            }
        }
    }
}
