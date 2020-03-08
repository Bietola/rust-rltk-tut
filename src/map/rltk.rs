use rltk::{Algorithm2D, BaseMap, Point};
use crate::map::base::Map;

/// Needed for interoperability with rltk
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

/// Needed for interoperability with rltk
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        !self.tiles[idx as usize].is_passable()
    }
}
