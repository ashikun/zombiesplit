///! Presenter state for individual splits.
use super::super::cursor::SplitPosition;
use std::fmt::Display;

use crate::model::{
    aggregate,
    attempt::observer::{split, time},
    comparison::pace::{self, PacedTime},
    time::position,
};

/// Presenter state about one split.
#[derive(Debug, Default, Clone)]
pub struct Split {
    /// The number of times that have been logged on this split.
    pub num_times: usize,
    /// The display name of the split.
    pub name: String,
    /// The aggregate times logged for this split.
    pub aggregates: aggregate::Full,
    /// The pace of this split in the run-so-far.
    pub pace_in_run: pace::SplitInRun,
    /// The last logged cursor-relative position for this split.
    pub position: SplitPosition,
    /// Any editor active on this split.
    pub editor: Option<super::Editor>,
}

impl Split {
    /// Creates a new split with a display name, but no other data logged.
    ///
    /// ```
    /// use zombiesplit::ui::presenter::state;
    ///
    /// let s = state::Split::new("Palmtree Panic 1");
    /// assert_eq!("Palmtree Panic 1", s.name);
    /// assert_eq!(0, s.num_times);
    /// assert_eq!(zombiesplit::model::comparison::pace::SplitInRun::Inconclusive, s.pace_in_run);
    /// ```
    pub fn new<N: Display>(name: N) -> Self {
        let name = name.to_string();
        Self {
            name,
            ..Self::default()
        }
    }

    /// Resets the per-run state of this split.
    ///
    /// This clears the aggregates, pacing information, and time count; it
    /// doesn't reset metadata.
    pub fn reset(&mut self) {
        self.num_times = 0;
        self.aggregates = aggregate::Full::default();
        self.pace_in_run = pace::SplitInRun::default();
    }

    /// Gets the cumulative time at this split along with its pace note.
    #[must_use]
    pub fn paced_cumulative(&self) -> PacedTime {
        let time = self.aggregates[aggregate::Source::Attempt][aggregate::Scope::Cumulative];
        PacedTime {
            pace: self.pace_in_run.overall(),
            time,
        }
    }

    /// Handles an observation for this split.
    pub fn handle_event(&mut self, evt: split::Event) {
        match evt {
            split::Event::Time(t, time::Event::Aggregate(kind)) => {
                self.aggregates[kind.source][kind.scope] = t;
            }
            split::Event::Time(_, time::Event::Pushed) => {
                self.num_times += 1;
            }
            split::Event::Time(_, time::Event::Popped) => {
                self.num_times -= 1;
                // Moving the newly popped time to the editor gets handled
                // elsewhere.
            }
            split::Event::Pace(pace) => {
                self.pace_in_run = pace;
            }
        }
    }

    /// Populates this split state with the current state of `editor`.
    ///
    /// # Panics
    ///
    /// If the field position is set to hours.
    pub fn set_editor(&mut self, editor: Option<&super::super::mode::Editor>) {
        self.editor = editor.map(|e| {
            let mut out = super::Editor {
                mins: e.time.mins.to_string(),
                secs: e.time.secs.to_string(),
                msecs: e.time.millis.to_string(),
                field: None,
            };

            if let Some(ref field) = e.field {
                let pos = field.position();
                out.field = Some(pos);
                let target = match pos {
                    position::Name::Minutes => &mut out.mins,
                    position::Name::Seconds => &mut out.secs,
                    position::Name::Milliseconds => &mut out.msecs,
                    position::Name::Hours => unimplemented!("hours"),
                };
                *target = field.to_string();
            }

            out
        });
    }
}
