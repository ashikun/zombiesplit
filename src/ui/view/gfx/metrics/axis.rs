/// Two-dimensional axes.
use super::Size;

/// Enumerates the axes in a `Size` or `Rect`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Axis {
    /// Horizontal orientation.
    Horizontal,
    /// Vertical orientation.
    Vertical,
}

impl Axis {
    /// Stacks `sizes` along this axis.
    #[must_use]
    pub fn stack_many(&self, sizes: impl IntoIterator<Item = Size>) -> Size {
        sizes
            .into_iter()
            .reduce(self.stack_function())
            .unwrap_or_default()
    }

    /// Gets the appropriate size stacking function for this axis.
    #[must_use]
    pub fn stack_function(&self) -> fn(Size, Size) -> Size {
        match self {
            Axis::Horizontal => Size::stack_horizontally,
            Axis::Vertical => Size::stack_vertically,
        }
    }

    /// Constructs a `Size` such that this axis has length `this_axis`, and the axis that is at a
    /// right angle to it has length `normal_axis`.
    #[must_use]
    pub const fn size(&self, this_axis: super::Length, normal_axis: super::Length) -> Size {
        match self {
            Axis::Horizontal => Size {
                w: this_axis,
                h: normal_axis,
            },
            Axis::Vertical => Size {
                w: normal_axis,
                h: this_axis,
            },
        }
    }

    /// Gets the axis at a right angle to this axis.
    ///
    /// ```
    /// use zombiesplit::ui::view::gfx::metrics::{Axis};
    ///
    /// assert_eq!(Axis::Vertical, Axis::Horizontal.normal());
    /// assert_eq!(Axis::Horizontal, Axis::Vertical.normal());
    /// ```
    #[must_use]
    pub const fn normal(&self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

/// [Size]s may be indexed by axis.
///
/// ```
/// use zombiesplit::ui::view::gfx::metrics::{Axis, Size};
///
/// let size = Size{ w: 19, h: 69 };
/// assert_eq!(size.w, size[Axis::Horizontal]);
/// assert_eq!(size.h, size[Axis::Vertical]);
/// ```
impl std::ops::Index<Axis> for Size {
    type Output = super::Length;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::Horizontal => &self.w,
            Axis::Vertical => &self.h,
        }
    }
}

/// [Size]s may be indexed mutably by axis.
impl std::ops::IndexMut<Axis> for Size {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::Horizontal => &mut self.w,
            Axis::Vertical => &mut self.h,
        }
    }
}

/// [Point]s may be indexed by axis.
///
/// ```
/// use zombiesplit::ui::view::gfx::metrics::{Axis, Point};
///
/// let point = Point{ x: 19, y: 69 };
/// assert_eq!(point.x, point[Axis::Horizontal]);
/// assert_eq!(point.y, point[Axis::Vertical]);
/// ```
impl std::ops::Index<Axis> for super::Point {
    type Output = super::point::Coord;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::Horizontal => &self.x,
            Axis::Vertical => &self.y,
        }
    }
}

/// [Point]s may be indexed mutably by axis.
impl std::ops::IndexMut<Axis> for super::Point {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::Horizontal => &mut self.x,
            Axis::Vertical => &mut self.y,
        }
    }
}
