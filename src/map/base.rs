use rltk::{Rltk, Console, RGB};
use num::ToPrimitive;

/*********/
/* Types */
/*********/

#[derive(PartialEq, Copy, Clone)]
/// A map tile
pub enum Tile {
    Wall, Floor,
}

/// The map
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    /// New map made entrirely from specified tile
    pub fn all(width: usize, height: usize, tile: Tile) -> Self {
        Map {
            width,
            height,
            tiles: vec![tile; width * height],
        }
    }

    /// New map made entrirely from specified tile
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

    /// Go from cartesian coordinates to map tile index
    pub fn xy_idx<I>(&self, x: I, y: I) -> usize
    where
        I: ToPrimitive
    {
        y.to_usize().unwrap() * self.width + x.to_usize().unwrap()
    }

    /// Get tile at specified position
    pub fn at<I>(&self, x: I, y: I) -> Tile
    where
        I: ToPrimitive
    {
        self.tiles[self.xy_idx(x, y)]
    }

    /// Get tile at specified position (mutable)
    pub fn at_mut<I>(&mut self, x: I, y: I) -> &mut Tile
    where
        I: ToPrimitive
    {
        let idx = self.xy_idx(x, y);
        &mut self.tiles[idx]
    }

    /// Draw the map
    pub fn draw(&self, ctx: &mut Rltk) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.at(x, y);

                let glyph = match tile {
                    Tile::Floor => rltk::to_cp437(' '),
                    Tile::Wall => rltk::to_cp437('#'),
                };

                ctx.set(x as i32, y as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), glyph);
            }
        }
    }
}
