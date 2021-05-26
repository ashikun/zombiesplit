//! Presenter-level types for handling splits.

use crate::model::run::Split;
use std::cmp::Ordering;

/// A split reference, containing position information the split.
#[derive(Copy, Clone)]
pub struct Ref<'a> {
    /// The index of the split reference.
    pub index: usize,
    /// A reference to the parent presenter.
    pub presenter: &'a super::Presenter,
    /// The split data.
    pub split: &'a Split,
}

impl<'a> Ref<'a> {
    /// Gets whether this split is currently active.
    #[must_use]
    pub fn position(&self) -> Position {
        self.presenter
            .mode
            .cursor_pos()
            .map_or(Position::Coming, |c| match self.index.cmp(&c) {
                Ordering::Less => Position::Done,
                Ordering::Equal => Position::Cursor,
                Ordering::Greater => Position::Coming,
            })
    }
}

/// Relative positions of splits to cursors.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Position {
    /// This split is before the cursor.
    Done,
    /// This split is on the cursor.
    Cursor,
    /// This split is after the cursor.
    Coming,
}