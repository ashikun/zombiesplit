//! The cursor struct and associated functionality.

/// A bounded cursor.
#[derive(Copy, Clone)]
pub struct Cursor {
    /// The current position.
    pos: usize,
    /// The maximum position.
    max: usize
}

impl Cursor {
    /// Creates a new cursor at the top, with the given maximum index.
    pub fn new(max: usize) -> Self {
        Self{pos: 0, max}
    }

    /// Gets the current cursor position.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Moves the cursor up by `n`, returning whether the position changed.
    pub fn move_up(&mut self, n: usize) -> bool {
        if self.pos == 0 {
            false
        } else if self.pos <= n {
            self.pos = 0;
            true
        } else {
            self.pos -= n;
            true
        }
    }

    /// Moves the cursor down by `n`, returning whether the position changed.
    pub fn move_down(&mut self, n: usize) -> bool {
        if self.pos == self.max {
            false
        } else if self.max <= self.pos + n {
            // TODO(@MattWindsor91): rewrite this to avoid overflow potential?
            self.pos = self.max;
            true
        } else {
            self.pos += n;
            true
        }
    }

    /// Moves the cursor in the direction of `m` `multiplier` times.
    pub fn move_by(&mut self, m: Motion, multiplier: usize) -> bool {
        // TODO(@MattWindsor91): multipliers
        match m {
            Motion::Up => self.move_up(multiplier),
            Motion::Down => self.move_down(multiplier),
        }
    }
}

/// A cursor motion.
#[derive(Copy, Clone)]
pub enum Motion {
    /// Move the cursor up.
    Up,
    /// Move the cursor down.
    Down,
}
