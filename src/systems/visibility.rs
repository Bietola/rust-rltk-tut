use crate::components as cmp;
use specs::prelude::*;
use crate::map::base::*;
use rltk::Point;
use crate::utils::rect::Rect;

/// Updates viewsheds using position and game map info.
struct Visibility;

impl<'a> System<'a> for Visibility {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, cmp::Viewshed>,
        ReadStorage<'a, cmp::Pos>,
    );

    fn run(&mut self, (map, mut viewshed, pos): Self::SystemData) {
        // Update viewsheds.
        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            // Retrieve normal reference to map from the ReadExpect thing.
            let map = &*map;

            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = rltk::field_of_view(Point::new(pos.x, pos.y), viewshed.range, map);

            // The viewshed might go behond the map borders.
            viewshed.visible_tiles.retain(|p| map.contains_point(p.x, p.y));
        }
    }
}
