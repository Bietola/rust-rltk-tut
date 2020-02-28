use crate::map::base::*;
use crate::utils::{Advance, Dir};
use derive_builder::Builder;
use itertools::Itertools;
use rand::Rng;

/// Generate random ugly map
pub fn make_ugly_map(width: usize, height: usize) -> Map {
    let mut res = Map::empty(width, height);

    // Border walls
    for i in 0..width {
        *res.at_mut(i, 0) = Tile::Wall;
        *res.at_mut(i, height - 1) = Tile::Wall;
    }
    for i in 0..height {
        *res.at_mut(0, i) = Tile::Wall;
        *res.at_mut(width - 1, i) = Tile::Wall;
    }

    // Generate some random walls
    // TODO: try to not generate walls over the player
    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(1, (width - 1) as i32);
        let y = rng.roll_dice(1, (height - 1) as i32);

        *res.at_mut(x, y) = Tile::Wall;
    }

    res
}

/// Simple Rooms 'n' Corridors Generator
mod rnc {
    use super::*;

    struct FailureInfo {
        pub partial_map: Map,
        pub rooms_left: u32,
    }

    #[derive(Builder)]
    pub struct Config {
        map_w: usize,
        map_h: usize,
        room_chance: f32,
        turn_chance: f32,
        min_room_size: u32,
        max_room_size: u32,
        // TODO: add corridor length and room connection options
        // NB. for now all rooms are connected
    }

    /// Create new simple map with rooms and corridors (Moria style)
    fn make_map(conf: Config) -> Result<Map, FailureInfo> {
        let res = Map::all(conf.map_w, conf.map_h, Tile::Wall);

        // Create rooms
        let mut rng = rand::thread_rng();
        let mut cur_dir = Dir::South;
        let mut cur_x = 0;
        let mut cur_y = 0;
        loop {
            // Generate room die roll
            if rng.gen::<f32>() < conf.room_chance {
                // Build room with corridor pointing at center
                let new_room = {
                    // Choose random room parameters
                    let width = rng.gen_range(conf.min_room_size, conf.max_room_size) as i32;
                    let height = rng.gen_range(conf.min_room_size, conf.max_room_size) as i32;

                    Room {
                        x: cur_x - width / 2,
                        y: cur_y - height / 2,
                        width,
                        height,
                    }
                };

                // TODO: Offset room randomly

                //Try to add room to map (ignore failure and continue)
                let _ignore = res.add_room(new_room);
            }
            // Turn corridor
            else if rng.gen::<f32>() < conf.turn_chance {
                cur_dir = Dir::cycle(cur_dir);
            }

            // Advance corridor in current position
            let (cur_x, cur_y) = (cur_x, cur_y).advance(cur_dir, 1);
        }
    }
}
