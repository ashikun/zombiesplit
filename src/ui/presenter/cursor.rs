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

    /// Creates a new cursor at `pos`, with the maximum index `max`.
    /// Returns `None` if `max < pos`.
    #[must_use]
    pub fn new_at(pos: usize, max: usize) -> Option<Self> {
        if max < pos {
            None
        } else {
            Some(Self { pos, max })
        }
    }

    /// Gets the current cursor position.
    #[must_use]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Gets the relative position of `split` to this cursor.
    #[must_use]
    pub fn split_position(&self, split: usize) -> SplitPosition {
        SplitPosition::from(split.cmp(&self.pos))
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

impl SplitPosition {
    /// Constructs a new split position by comparing a split position
    /// `split_pos` to an optional cursor position `cur_pos`.
    ///
    /// # Example
    ///
    /// ```
    /// use zombiesplit::ui::presenter::cursor::SplitPosition;
    ///
    /// // no cursor is given
    /// assert_eq!(SplitPosition::default(), SplitPosition::new(0, None));
    ///
    /// // split is before the cursor
    /// assert_eq!(SplitPosition::Done, SplitPosition::new(0, Some(5)));
    ///
    /// // split is on the cursor
    /// assert_eq!(SplitPosition::Cursor, SplitPosition::new(5, Some(5)));
    ///
    /// // split is after the cursor
    /// assert_eq!(SplitPosition::Coming, SplitPosition::new(10, Some(5)));
    /// ```
    #[must_use]
    pub fn new(split_pos: usize, cur_pos: Option<usize>) -> Self {
        cur_pos.map_or(SplitPosition::default(), |cur_pos| {
            SplitPosition::from(split_pos.cmp(&cur_pos))
        })
    }
}

/// Converting an ordering between split (on LHS) and cursor (on RHS) positions
/// into a relative position for the split.
///
/// # Example
///
/// ```
/// use zombiesplit::ui::presenter::cursor::SplitPosition;
///
/// // split is before the cursor
/// assert_eq!(SplitPosition::Done, SplitPosition::from(0.cmp(&5)));
///
/// // split is on the cursor
/// assert_eq!(SplitPosition::Cursor, SplitPosition::from(5.cmp(&5)));
///
/// // split is after the cursor
/// assert_eq!(SplitPosition::Coming, SplitPosition::from(10.cmp(&5)));
/// ```
impl From<Ordering> for SplitPosition {
    fn from(split_cmp_cur: std::cmp::Ordering) -> Self {
        match split_cmp_cur {
            Ordering::Less => Self::Done,
            Ordering::Equal => Self::Cursor,
            Ordering::Greater => Self::Coming,
        }
    }
}
