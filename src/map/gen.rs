use crate::map::base::*;
use crate::utils::{
    dir::{Advance, Dir},
    rect::*,
};
use derive_builder::Builder;
use log::{error, info, warn};
use rand::Rng;

/// Generate random ugly map
#[allow(dead_code)]
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
pub mod rnc {
    use super::*;

    #[derive(Builder)]
    #[builder(default)]
    pub struct Config {
        map_width: usize,
        map_height: usize,
        room_chance: f32,
        turn_chance: f32,
        min_room_size: u32,
        max_room_size: u32,
        iterations: u32,
        // TODO: add corridor length and room connection options
        // NB. for now all rooms are connected
    }

    impl Default for Config {
        fn default() -> Self {
            Config {
                map_width: 100,
                map_height: 100,
                room_chance: 1.,
                turn_chance: 1.,
                min_room_size: 4,
                max_room_size: 10,
                iterations: 1000,
            }
        }
    }

    /// Create new simple map with rooms and corridors (Moria style)
    pub fn make_map(conf: Config) -> Result<Map, Map> {
        info!("STARTING NEW R&C DUNGEON GENERATION PROCESS");

        // Start with map filled with walls
        let mut res = Map::all(conf.map_width, conf.map_height, Tile::Wall);

        let mut rng = rand::thread_rng();

        // Corridor starting state (random/arbitrary).
        let mut cur_dir = Dir::South;
        let mut cur_x: i32 = rng.gen_range(1, (conf.map_width - 1) as i32);
        let mut cur_y: i32 = rng.gen_range(1, (conf.map_height - 1) as i32);

        // Start creating rooms and corridors!
        for _ in 0..conf.iterations {
            // Carve corridor.
            *res.at_mut(cur_x, cur_y) = Tile::Floor;

            // Generate room if chances are right.
            if rng.gen_range(0., 100.) < conf.room_chance {
                // Build room with corridor pointing at center.
                info!("Room roll successfull");
                let new_room = {
                    // Choose random room parameters.
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

                //Try to add room to map (ignore failure and continue).
                info!(
                    "Spawning room: {:?} [corridor pos: ({}, {})].",
                    new_room, cur_x, cur_y
                );
                if !res.add_room(new_room) {
                    warn!("FAILED room spawn, skipping...");
                }
            }
            // Change corridor generation direction if chances are right.
            else if rng.gen_range(0., 100.) < conf.turn_chance {
                cur_dir = Dir::cycle(cur_dir);
                info!("Corridor turn roll successful. New corridor advancement direction: {:?} [corridor pos: ({}, {})].", cur_dir, cur_x, cur_y);
            }

            // Advance corridor in current position; checking if the new position is valid.
            for tries in 0..4 {
                // Stop generation prematurely on third try.
                // TODO: Give option to create dead ends.
                if tries == 3 {
                    warn!("Too many corridor advancement attempts... returning partial map.");
                    return Err(res);
                }

                // Speculate new position (might be changed) if not valid.
                let (new_x, new_y) = (cur_x, cur_y).advance(cur_dir, 1);

                // Change direction and retry if touching the boundary walls (the map's outer
                // frame).
                if new_y == 49 {
                    println!("Evaluating pos OOB: {}, {}", new_x, new_y);
                }
                let corridor_oob = !res
                    .trim_outer_frame(1)
                    .and_then(|t| {
                        println!("{:?}", t);
                        Some(t)
                    })
                    .unwrap_or_else(|| {
                        error!("FAILED! Map is too small...");
                        panic!(
                            "Map shouldn't absolutely be this small: {}x{}",
                            conf.map_width, conf.map_height
                        )
                    })
                    .contains_point(new_x, new_y);
                if corridor_oob {
                    if new_y == 49 {
                        println!("SUCCESS");
                    }
                    info!("Corridor failed to advance (OOB)!");
                    cur_dir = cur_dir.cycle();
                    continue;
                }

                // TODO: make this work by placing corridor in other place.
                // Do the same if about to enter a room.
                // for room in &res.rooms {
                //     if room.add_outer_frame(1).contains_point(new_x, cur_y) {
                //         cur_dir = cur_dir.cycle();
                //         continue;
                //     }
                // }

                // No problems with new corridor position... perform the advancement.
                cur_x = new_x;
                cur_y = new_y;
                break;
            }
        }

        // All fine
        Ok(res)
    }
}
