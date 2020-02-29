use crate::map::{Map, Room};
use num_traits::{NumAssign, NumOps};

/// A utility trait with all the needed contraints for an element to be used to describe the
/// dimensions of a `Rect`.
///
/// TODO: Use a macro to avoid the duplicated list of traits.
pub trait RectEle: NumOps + NumAssign + Ord + Sized + Copy {}
impl<T> RectEle for T where T: NumOps + NumAssign + Ord + Sized + Copy {}

#[derive(PartialEq, Debug, Clone)]
/// A rectangle described by its four corners.
pub struct SimpleRect<T: RectEle> {
    x: T,
    y: T,
    w: T,
    h: T,
}

impl<T: RectEle> From<(T, T, T, T)> for SimpleRect<T> {
    fn from((x, y, w, h): (T, T, T, T)) -> Self {
        Self { x, y, w, h }
    }
}

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

        x >= rx && x <= rx + rw && y >= ry && y <= ry + rh
    }

    /// Checks if rect contains a given second rect.
    fn contains_rect<R, E>(&self, inner_rect: R) -> bool
    where
        E: RectEle + Into<T>,
        R: Rect<E>,
    {
        let (ix, iy, iw, ih) = inner_rect.get_corners();

        // TODO: Maybe use a macro to iterate over the tuple...
        let (ix, iy, iw, ih): (T, T, T, T) = (ix.into(), iy.into(), iw.into(), ih.into());
        let (ox, oy, ow, oh) = self.get_corners();

        ix >= ox && ix + iw <= ox + ow && iy >= oy && iy + ih <= oy + oh
    }
}

/// SimpleRect is obviously a Rect.
impl<T> Rect<T> for SimpleRect<T>
where
    T: RectEle,
{
    fn get_corners(&self) -> (T, T, T, T) {
        (self.x, self.y, self.w, self.h)
    }
}

/// Utility impl for Map.
impl Rect<usize> for Map {
    fn get_corners(&self) -> (usize, usize, usize, usize) {
        (0, 0, self.width, self.height)
    }
}

/// Utility impl for Room.
impl Rect<i32> for Room {
    fn get_corners(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }
}

/// Automatic impl for references of structs for which `Rect` is already impl-ed.
impl<T, R> Rect<T> for &R
where
    T: RectEle,
    R: Rect<T>,
{
    fn get_corners(&self) -> (T, T, T, T) {
        (*self).get_corners()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_outer_frame_1_to_4x5() {
        let r = SimpleRect::from((0, 0, 4, 5));

        assert_eq!(
            SimpleRect::from((1, 1, 3, 4)),
            r.trim_outer_frame(1).unwrap(),
        )
    }

    #[test]
    fn contains_point_bottom_right() {
        let r = SimpleRect::from((0, 0, 4, 5));
        let p = (4, 5);

        assert!(r.contains_point(p.0, p.1))
    }

    #[test]
    fn add_outer_frame_1_to_4x5() {
        let r = SimpleRect::from((1, 1, 5, 6));

        assert_eq!(SimpleRect::from((0, 0, 6, 7)), r.add_outer_frame(1),)
    }

    #[test]
    fn contains_point_rect_contains_all_its_points() {
        let r = SimpleRect::from((0, 2, 6, 12));

        for x in r.x..=(r.x + r.w) {
            for y in r.y..=(r.y + r.h) {
                assert!(r.contains_point(x, y))
            }
        }
    }

    #[test]
    fn contains_point_rect_does_not_contain_its_outer_frame() {
        let r = SimpleRect {
            x: 1,
            y: 2,
            w: 6,
            h: 12,
        };

        for &x in &[r.x - 1, r.x + r.w + 1] {
            for y in r.y..=(r.y + r.h) {
                println!("{}, {}", x, y);
                assert!(!r.contains_point(x, y));
            }
        }

        for &y in &[r.y - 1, r.y + r.h + 1] {
            for x in r.x..=(r.x + r.w) {
                println!("{}, {}", x, y);
                assert!(!r.contains_point(x, y));
            }
        }
    }

    #[test]
    fn contains_rect_rect_contains_itself() {
        let r = SimpleRect {
            x: 1,
            y: 2,
            w: 6,
            h: 12,
        };

        assert!(r.contains_rect(&r));
    }
}
