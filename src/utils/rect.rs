use crate::map::{Map, Room};

#[derive(PartialEq, Debug, Clone)]
/// A rectangle described by its four corners.
pub struct SimpleRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl From<(i32, i32, i32, i32)> for SimpleRect {
    fn from((x, y, w, h): (i32, i32, i32, i32)) -> Self {
        Self { x, y, w, h }
    }
}

/// A useful trait for rectangle-like things.
pub trait Rect {
    /// In this order: x and y of top left corner fallowed by width and height.
    fn get_corners(&self) -> (i32, i32, i32, i32);

    /// Trim outer border of rectangle.
    fn trim_outer_frame(&self, amount: i32) -> Option<SimpleRect> {
        let (mut x, mut y, mut w, mut h) = self.get_corners();

        x += amount;
        y += amount;
        w -= amount * 2;
        h -= amount * 2;

        // No rectangles taking up negative space are created...
        if w < 0 || h < 0 {
            None
        } else {
            Some(SimpleRect { x, y, w, h })
        }
    }

    /// Wrap the rectangle in an outer frame of `thickness` thickness.
    fn add_outer_frame(&self, thickness: i32) -> SimpleRect {
        let (mut x, mut y, mut w, mut h) = self.get_corners();

        x -= thickness;
        y -= thickness;
        w += thickness;
        h += thickness;

        SimpleRect { x, y, w, h }
    }

    /// Checks if rect contains a given point.
    fn contains_point(&self, x: i32, y: i32) -> bool
    where
        // TODO: find out why this is needem
        Self: Sized,
    {
        let (rx, ry, rw, rh) = self.get_corners();

        x >= rx && x < rx + rw && y >= ry && y < ry + rh
    }

    /// Checks if rect contains a given second rect.
    fn contains_rect<R>(&self, inner_rect: R) -> bool
    where
        R: Rect,
    {
        let (ix, iy, iw, ih) = inner_rect.get_corners();
        let (ox, oy, ow, oh) = self.get_corners();

        ix >= ox && ix + iw <= ox + ow && iy >= oy && iy + ih <= oy + oh
    }
}

/// SimpleRect is obviously a Rect.
impl Rect for SimpleRect {
    fn get_corners(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.w, self.h)
    }
}

/// Utility impl for Map.
impl Rect for Map {
    fn get_corners(&self) -> (i32, i32, i32, i32) {
        (0, 0, self.width as i32, self.height as i32)
    }
}

/// Utility impl for Room.
impl Rect for Room {
    fn get_corners(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }
}

/// Automatic impl for references of structs for which `Rect` is already impl-ed.
impl<R> Rect for &R
where
    R: Rect,
{
    fn get_corners(&self) -> (i32, i32, i32, i32) {
        (*self).get_corners()
    }
}

/// Any reference to rect-like thing can be turned into a `SimpleRect`
impl<R> From<&R> for SimpleRect
where
    R: Rect,
{
    fn from(rect: &R) -> Self {
        SimpleRect::from(rect.get_corners())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_outer_frame_1_to_4x5() {
        let r = SimpleRect::from((0, 0, 4, 5));

        assert_eq!(
            SimpleRect::from((1, 1, 2, 3)),
            r.trim_outer_frame(1).unwrap(),
        )
    }

    #[test]
    fn contains_point_bottom_right() {
        let r = SimpleRect::from((0, 0, 4, 5));
        let p = (3, 4);

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

        for x in r.x..(r.x + r.w) {
            for y in r.y..(r.y + r.h) {
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
