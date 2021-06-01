//! Structs and functions for pace computation.

/// Possible paces for a split or run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pace {
    /// There is no pacing time.
    Inconclusive,
    /// This split, or run, is behind its comparison.
    Behind,
    /// This split, or run, is ahead its comparison.
    Ahead,
    /// This split, or run, is a personal best ('gold split').
    PersonalBest,
}

/// The default pace is inconclusive.
impl Default for Pace {
    fn default() -> Self {
        Self::Inconclusive
    }
}

/// A pair of a time and its pace against comparison.
#[derive(Clone, Copy, Debug, Default)]
pub struct PacedTime {
    /// The pace.
    pub pace: Pace,
    /// The time to which `pace` applies.
    pub time: super::time::Time,
}

/// A pair of split pace and run-so-far pace.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pair {
    /// The split pace.
    pub split: PacedTime,
    /// The run-so-far pace.
    pub run_so_far: PacedTime,
}
