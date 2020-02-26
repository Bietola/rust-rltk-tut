use rltk::RandomNumberGenerator;
use num::{ToPrimitive, Integer};

/*************/
/* Constants */
/*************/
pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

/*********/
/* Types */
/*********/
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor,
}

/// A map is just a vector of tiles
type Map = Vec<TileType>;

/*******************/
/* Utility methods */
/*******************/
/// Go from cartesian coordinates to map tile index
pub fn xy_idx<I>(x: I, y: I) -> usize
where
    I: ToPrimitive
{
    y.to_usize().unwrap() * MAP_WIDTH + x.to_usize().unwrap()
}

/*******************/
/* Map generations */
/*******************/
/// Generate empty map
pub fn empty_map() -> Map {
    vec![TileType::Floor; MAP_HEIGHT]
}

/// Generate random map
pub fn new_map() -> Map {
    let mut result = empty_map();

    // Border walls
    for i in 0..MAP_WIDTH {
        result[xy_idx(i, 0)] = TileType::Wall;
        result[xy_idx(i, MAP_HEIGHT - 1)] = TileType::Wall;
    }
    for i in 0..MAP_HEIGHT {
        result[xy_idx(0, i)] = TileType::Wall;
        result[xy_idx(MAP_WIDTH - 1, i)] = TileType::Wall;
    }

    // Generate some random walls
    // TODO: try to not generate walls over the player
    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(0, MAP_WIDTH as i32);
        let y = rng.roll_dice(0, MAP_HEIGHT as i32);

        result[xy_idx(x, y)] = TileType::Wall;
    }
    
    result
}
