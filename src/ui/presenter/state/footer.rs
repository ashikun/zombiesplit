/*! State pertaining to the footer widgets of the split view. */

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use crate::model::{
    session,
    timing::{comparison, Time},
};

/// Presenter state used in the footer widget.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Footer {
    /// The total time of the run up to the cursor, and its pace.
    pub at_cursor: comparison::pace::PacedTime,

    /// The total time of the run, and its pace.
    pub total: comparison::pace::PacedTime,

    /// Comparison times for this run.
    pub comparisons: comparison::Run,
}

impl Footer {
    /// Constructs a footer from a session state dump.
    #[must_use]
    pub fn from_dump(dump: &session::State) -> Self {
        Self {
            at_cursor: comparison::pace::PacedTime::default(),
            // TODO(@MattWindsor91): fix this
            total: comparison::pace::PacedTime::default(),
            comparisons: dump.comparison.run,
        }
    }

    /// Gets a specific row from the footer programmatically.
    #[must_use]
    pub fn get(&self, row: RowType) -> Option<Cow<comparison::pace::PacedTime>> {
        match row {
            RowType::Comparison => lift_comparison(&self.comparisons.total_in_pb_run),
            RowType::SumOfBest => lift_comparison(&self.comparisons.sum_of_best),
            RowType::UpToCursor => Some(Cow::Borrowed(&self.at_cursor)),
            RowType::Total => Some(Cow::Borrowed(&self.total)),
        }
    }
}

/// Lifts an optional comparison `time` to a copy-on-write paced time.
fn lift_comparison(time: &Option<Time>) -> Option<Cow<comparison::pace::PacedTime>> {
    time.map(|x| Cow::Owned(comparison::pace::PacedTime::inconclusive(x)))
}

/// Enumeration of types of row in the totals box.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum RowType {
    /// The comparison time.
    Comparison,
    /// The attempt total.
    Total,
    /// The attempt total as far as the current cursor.
    UpToCursor,
    /// The sum of best.
    SumOfBest,
}

/// Produces the label names for each row type.
impl Display for RowType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl RowType {
    /// Gets the label associated with this row type.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Total => "Total",
            Self::Comparison => "Comparison",
            Self::UpToCursor => "Up to cursor",
            Self::SumOfBest => "Sum of best",
        }
    }
}
