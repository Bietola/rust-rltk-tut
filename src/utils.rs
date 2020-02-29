use crate::map::base::*;
use num_traits::{NumAssign, NumOps};
use std::ops::{Add, Sub};

/*************/
/* Rectangle */
/*************/

/// A utility trait with all the needed contraints for an element to be used to describe the
/// dimensions of a `Rect`.
///
/// TODO: Use a macro to avoid the duplicated list of traits.
pub trait RectEle: NumOps + NumAssign + Ord + Sized + Copy {}
impl<T> RectEle for T where T: NumOps + NumAssign + Ord + Sized + Copy {}

/// A useful trait for rectangle-like things.
pub trait Rect<T: RectEle> {
    /// In this order: x and y of top left corner fallowed by width and height.
    fn get_corners(&self) -> (T, T, T, T);

    /// Trim outer border of rectangle.
    fn trim_outer_frame(&self, amount: T) -> Option<SimpleRect<T>> {
        let (mut x, mut y, mut w, mut h) = self.get_corners();

        x += amount;
        y += amount;
        w -= amount;
        h -= amount;

        // No rectangles taking up negative space are created...
        if w < T::zero() || h < T::zero() {
            None
        } else {
            Some(SimpleRect { x, y, w, h })
        }
    }

    /// Wrap the rectangle in an outer frame of `thickness` thickness.
    fn add_outer_frame(&self, thickness: T) -> SimpleRect<T> {
        let (mut x, mut y, mut w, mut h) = self.get_corners();

        x -= thickness;
        y -= thickness;
        w += thickness;
        h += thickness;

        SimpleRect { x, y, w, h }
    }

    /// Checks if rect contains a given point.
    fn contains_point(&self, x: T, y: T) -> bool {
        let (rx, ry, rw, rh) = self.get_corners();

        x > rx && x < rx + rw && y > ry && y < ry + rh
    }
}

/// A rectangle described by its four corners.
pub struct SimpleRect<T: RectEle> {
    x: T,
    y: T,
    w: T,
    h: T,
}

impl<T: RectEle> Rect<T> for SimpleRect<T> {
    fn get_corners(&self) -> (T, T, T, T) {
        (self.x, self.y, self.w, self.h)
    }
}

impl Rect<usize> for Map {
    fn get_corners(&self) -> (usize, usize, usize, usize) {
        (0, 0, self.width, self.height)
    }
}

impl Rect<i32> for Room {
    fn get_corners(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }
}

/*************/
/* Direction */
/*************/

#[derive(Clone, Copy, PartialEq)]
/// Your run of the mill cardinal direction
pub enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    /// Cycle through all directions
    pub fn cycle(self) -> Self {
        use Dir::*;
        match self {
            North => South,
            South => West,
            West => East,
            East => North,
        }
    }
}

/// Trait for coordinate like things that can be moved in a particular direction
pub trait Advance<T> {
    fn advance(self, dir: Dir, steps: T) -> Self;
}

impl<T, R> Advance<R> for (T, T)
where
    T: Add<R, Output = T> + Sub<R, Output = T>,
    R: Copy,
{
    fn advance(self, dir: Dir, steps: R) -> Self {
        let (x, y) = self;
        (
            match dir {
                Dir::East => x + steps,
                Dir::West => x - steps,
                _ => x,
            },
            match dir {
                Dir::North => y + steps,
                Dir::South => y - steps,
                _ => y,
            },
        )
    }
}
