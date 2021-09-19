//! The cursor struct and associated functionality.

use std::cmp::Ordering;

/// A bounded cursor.
#[derive(Copy, Clone)]
pub struct Cursor {
    /// The current position.
    pos: usize,
    /// The maximum position.
    max: usize,
}

impl Cursor {
    /// Creates a new cursor at the top, with the given maximum index.
    #[must_use]
    pub fn new(max: usize) -> Self {
        Self { pos: 0, max }
    }

    /// Gets the current cursor position.
    #[must_use]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Gets the relative position of `split` to this cursor.
    #[must_use]
    pub fn split_position(&self, split: usize) -> SplitPosition {
        match split.cmp(&self.pos) {
            Ordering::Less => SplitPosition::Done,
            Ordering::Equal => SplitPosition::Cursor,
            Ordering::Greater => SplitPosition::Coming,
        }
    }

    /// Moves the cursor up by `n`, returning the amount by which the position moved.
    pub fn move_up(&mut self, n: usize) -> usize {
        if n <= self.pos {
            self.pos -= n;
            n
        } else {
            std::mem::replace(&mut self.pos, 0)
        }
    }

    /// Moves the cursor down by `n`, returning the amount by which the position moved.
    pub fn move_down(&mut self, n: usize) -> usize {
        let cap = self.max - self.pos;
        if n <= cap {
            self.pos += n;
            n
        } else {
            self.pos = self.max;
            cap
        }
    }

    /// Moves the cursor in the direction of `m` `multiplier` times.
    /// Returns the absolute amount by which the cursor moved.
    pub fn move_by(&mut self, m: Motion, multiplier: usize) -> usize {
        // TODO(@MattWindsor91): multipliers
        match m {
            Motion::Up => self.move_up(multiplier),
            Motion::Down => self.move_down(multiplier),
        }
    }
}

/// A cursor motion.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Motion {
    /// Move the cursor up.
    Up,
    /// Move the cursor down.
    Down,
}

/// Relative positions of splits to cursors.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum SplitPosition {
    /// This split is before the cursor.
    Done,
    /// This split is on the cursor.
    Cursor,
    /// This split is after the cursor.
    Coming,
}

/// By default, we consider splits to be coming after the cursor.
impl Default for SplitPosition {
    fn default() -> Self {
        Self::Coming
    }
}
