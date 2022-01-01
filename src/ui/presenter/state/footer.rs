/*! State pertaining to the footer widgets of the split view. */

use crate::model::{comparison::pace, Time};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

/// Presenter state used in the footer widget.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Footer {
    /// The total time of the run up to the cursor, and its pace.
    pub at_cursor: pace::PacedTime,

    /// The total time of the run, and its pace.
    pub total: pace::PacedTime,

    /// The target time of the run, if any.
    pub target: Option<Time>,
}

impl Footer {
    /// Gets a specific row from the footer programmatically.
    #[must_use]
    pub fn get(&self, row: RowType) -> Option<Cow<pace::PacedTime>> {
        match row {
            RowType::Comparison => {
                (&self.target).map(|x| Cow::Owned(pace::PacedTime::inconclusive(x)))
            }
            RowType::UpToCursor => Some(Cow::Borrowed(&self.at_cursor)),
            RowType::Total => Some(Cow::Borrowed(&self.total)),
        }
    }
}

/// Enumeration of types of row in the totals box.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RowType {
    /// The comparison time.
    Comparison,
    /// The attempt total.
    Total,
    /// The attempt total as far as the current cursor.
    UpToCursor,
}

/// Produces the label names for each row type.
impl Display for RowType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RowType::Total => "Total",
            RowType::Comparison => "Comparison",
            RowType::UpToCursor => "Up to cursor",
        })
    }
}
