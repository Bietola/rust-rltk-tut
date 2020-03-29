use crate::map::base::Map;
use rltk::{Algorithm2D, BaseMap, Point};

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

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let Point { x, y } = self.idx_xy(idx);

        vec![
            // Cardinals.
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            // Diagonals.
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ]
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        // Diagonals cost sqrt(2) to walk through.
        .map(|(x, y)| (self.xy_idx(x, y), if x == 0 || y == 0 { 1. } else { 1.42 }))
        .filter(|&(pos, _)| self.tiles[pos].is_passable())
        .collect()
    }
}
