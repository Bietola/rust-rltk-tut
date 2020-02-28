use num;
use std::ops::{Add, Sub};

/// Your run of the mill cardinal direction
#[derive(Clone, Copy, PartialEq)]
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
