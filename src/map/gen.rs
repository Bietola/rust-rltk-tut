use crate::map::base::*;
use derive_builder::Builder;

/*******************/
/* Test generators */
/*******************/

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

/****************************************/
/* Simple Rooms 'n' Corridors Generator */
/****************************************/

// mod rnc {
//     struct Rect {
//         x: i32,
//         y: i32,
//         w: i32,
//         h: i32,
//     }

//     #[derive(Builder)]
//     pub struct Config {
//         map_w: usize,
//         map_h: usize,
//         rooms_n: u32,
//         corridors_n: u32,
//         min_room_size: u32,
//         max_room_size: u32,
//         // TODO: add corridor length and room connection options
//         // NB. for now all rooms are connected
//     }

//     /// Create new simple map with rooms and corridors (Moria style)
//     fn new_map(conf: Config) -> Map {
//         let res = Map::all(Tile::Wall);
//         let mut rng = rltk::RandomNumberGenerator::new();

//         // Create rooms
//         for i in 0..100 {
//             place_rand_room();

//             rand_pos(map_w, map_h);
//         }
//     }
// }
