//! The cursor struct and associated functionality.

use std::cmp::Ordering;

/// A bounded cursor.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Cursor {
    /// The current position.
    pos: usize,
    /// The maximum position.
    max: usize,
}

impl Cursor {
    /// Constructs a new cursor with the given current and maximum position.
    ///
    /// If the current position is out of bounds of the maximum position, it will be silently
    /// clamped.
    ///
    /// ```
    /// use zombiesplit::ui::presenter::state::cursor::{Cursor, Motion};
    ///
    /// // The following are equivalent:
    /// let mut c1 = Cursor::default();
    /// c1.resize(10);
    /// c1.move_by(Motion::Down, 5);
    ///
    /// let c2 = Cursor::new(5, 10);
    ///
    /// assert_eq!(c1, c2);
    /// ```
    #[must_use]
    pub fn new(pos: usize, max: usize) -> Self {
        Self {
            pos: pos.min(max),
            max,
        }
    }

    /// Gets the current cursor position.
    #[must_use]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Resets the cursor to the top.
    ///
    /// Split positions, totals, etc. should be recalculated after doing this.
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    /// Sets the maximum index to `max`, moving the position upwards if necessary.
    ///
    /// Split positions, totals, etc. should be recalculated after doing this.
    ///
    /// ```
    /// use zombiesplit::ui::presenter::state::cursor::{Cursor, Motion};
    ///
    /// let mut c = Cursor::default();
    /// // By default, the cursor expects one position.
    /// assert_eq!(0, c.position());
    ///
    /// c.move_by(Motion::Down, 7);
    /// assert_eq!(0, c.position());
    ///
    /// c.resize(10);
    /// c.move_by(Motion::Down, 7);
    /// assert_eq!(7, c.position());
    ///
    /// // Resizing now will truncate the position:
    /// c.resize(5);
    /// assert_eq!(5, c.position());
    ///
    /// // But resizing up won't change it.
    /// c.resize(7);
    /// assert_eq!(5, c.position());
    /// ```
    pub fn resize(&mut self, max: usize) {
        self.max = max;
        self.pos = self.pos.min(max);
    }

    /// Gets the relative position of `split` to this cursor.
    #[must_use]
    pub fn split_position(&self, split: usize) -> SplitPosition {
        SplitPosition::from(split.cmp(&self.pos))
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

    /// Moves the cursor up by `n`, returning the amount by which the position moved.
    fn move_up(&mut self, n: usize) -> usize {
        if n <= self.pos {
            self.pos -= n;
            n
        } else {
            std::mem::replace(&mut self.pos, 0)
        }
    }

    /// Moves the cursor down by `n`, returning the amount by which the position moved.
    fn move_down(&mut self, n: usize) -> usize {
        let cap = self.max - self.pos;
        if n <= cap {
            self.pos += n;
            n
        } else {
            self.pos = self.max;
            cap
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
    /// use zombiesplit::ui::presenter::state::cursor::SplitPosition;
    ///
    /// // split is before the cursor
    /// assert_eq!(SplitPosition::Done, SplitPosition::new(0, 5));
    ///
    /// // split is on the cursor
    /// assert_eq!(SplitPosition::Cursor, SplitPosition::new(5, 5));
    ///
    /// // split is after the cursor
    /// assert_eq!(SplitPosition::Coming, SplitPosition::new(10, 5));
    /// ```
    #[must_use]
    pub fn new(split_pos: usize, cur_pos: usize) -> Self {
        SplitPosition::from(split_pos.cmp(&cur_pos))
    }
}

/// Converting an ordering between split (on LHS) and cursor (on RHS) positions
/// into a relative position for the split.
///
/// # Example
///
/// ```
/// use zombiesplit::ui::presenter::state::cursor::SplitPosition;
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Tests the clamping functionality of `new`.
    #[test]
    fn new_should_clamp() {
        let c = Cursor::new(10, 5);
        assert_eq!(c.pos, 5);
        assert_eq!(c.max, 5);
    }
}
