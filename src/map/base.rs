use crate::components as cmp;
use crate::utils::rect::Rect;
use num::ToPrimitive;
use rltk::{Console, Point, Rltk, RGB};

#[derive(PartialEq, Copy, Clone, Debug)]
/// A map tile
pub enum Tile {
    Wall,
    Floor,
}

impl Tile {
    pub fn is_passable(self) -> bool {
        self == Self::Floor
    }
}

/// A map room
#[derive(Debug)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn intersect(&self, other: &Room) -> bool {
        let bottom = self.y + self.height;
        let right = self.x + self.width;
        let other_bot = other.y + other.height;
        let other_right = other.x + other.width;

        ((other.y > self.y && other.y < bottom) || (self.y > other.y && self.y < other_bot))
            && ((other.x > self.x && other.x < right) || (self.x > other.x && self.x < other_right))
    }
}

/// The map
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Room>,
}

impl Map {
    /// New map made entrirely from specified tile
    pub fn all(width: usize, height: usize, tile: Tile) -> Self {
        Map {
            width,
            height,
            tiles: vec![tile; width * height],
            rooms: vec![],
        }
    }

    /// New map made entrirely from specified tile
    #[allow(dead_code)]
    pub fn empty(width: usize, height: usize) -> Self {
        Self::all(width, height, Tile::Floor)
    }

    /// Get the map width
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the map height
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Get iterator to internal tile storage
    #[allow(dead_code)]
    pub fn tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    /// Go from cartesian coordinates to map tile index
    pub fn xy_idx<I>(&self, x: I, y: I) -> usize
    where
        I: ToPrimitive,
    {
        y.to_usize().unwrap() * self.width + x.to_usize().unwrap()
    }

    /// Go from map idx to map cartesian coordinates
    pub fn idx_xy(&self, idx: usize) -> Point
    {
        Point {
            x: (idx % self.width) as i32,
            y: (idx / self.width) as i32,
        }
    }

    /// Get tile at specified position
    pub fn at<I>(&self, x: I, y: I) -> Tile
    where
        I: ToPrimitive + std::fmt::Debug,
    {
        if !self.contains_point(x.to_i32().unwrap(), y.to_i32().unwrap()) {
            panic!("Point ({:?}, {:?}) not contained by map!", x, y);
        }

        self.tiles[self.xy_idx(x, y)]
    }

    /// Get tile at specified position (mutable)
    pub fn at_mut<I>(&mut self, x: I, y: I) -> &mut Tile
    where
        I: ToPrimitive + std::fmt::Debug,
    {
        if !self.contains_point(x.to_i32().unwrap(), y.to_i32().unwrap()) {
            panic!("Point ({:?}, {:?}) not contained by map!", x, y);
        }

        let idx = self.xy_idx(x, y);
        &mut self.tiles[idx]
    }

    /// Tries to add rectangular room to map. Returns success as boolean.
    pub fn add_room(&mut self, new_room: Room) -> bool {
        // Cannot add room if it does not respect map bounds.
        // TODO: make this generic.
        if !self
            .trim_outer_frame(1)
            .expect("Map too small to be trimmed...")
            .contains_rect(&new_room)
        {
            return false;
        }

        // Cannot add room if it intersects with already existing rooms.
        for room in &self.rooms {
            if room.intersect(&new_room) {
                return false;
            }
        }

        // Carve room into map.
        for y in 0..new_room.height {
            for x in 0..new_room.width {
                *self.at_mut(new_room.x + x, new_room.y + y) = Tile::Floor;
            }
        }

        // Keep track of room structure.
        self.rooms.push(new_room);

        // All is fine.
        true
    }

    /// Draw the map.
    pub fn draw(&self, pl_viewshed: &cmp::Viewshed, ctx: &mut Rltk) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.at(x, y);

                let glyph = match tile {
                    Tile::Floor => rltk::to_cp437(' '),
                    Tile::Wall => rltk::to_cp437('#'),
                };

                // Draw only if visible by the player.
                // if pl_viewshed.visible_tiles.contains(&Point::new(x, y)) {
                    ctx.set(
                        x as i32,
                        y as i32,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::BLACK),
                        glyph,
                    );
                // }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idx_xy_simple() {
        // The test map.
        // *  0  1  2  3  4
        // 0  0  1  2  3  4
        // 1  5  6  7  8  9
        // 2 10 11 12 13 14
        let map = Map::empty(5, 3);

        // Test some random point.
        assert_eq!(map.idx_xy(5), Point::new(0, 1));
        assert_eq!(map.idx_xy(2), Point::new(2, 0));
        assert_eq!(map.idx_xy(14), Point::new(4, 2));
        assert_eq!(map.idx_xy(7), Point::new(2, 1));
        assert_eq!(map.idx_xy(3), Point::new(3, 0));
    }

    #[test]
    fn idx_xy_inv() {
        // TODO: eri qui CICCIO.
        unimplemented!();
    }
}
