mod components;
mod consts;
mod game_state;
mod map;
mod systems;
mod utils;

use crate::components as cmp;
use crate::game_state::State;
use consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use log4rs;
use map::gen::rnc;
use rand::seq::SliceRandom;
use rltk::RGB;
use specs::prelude::*;

fn main() {
    // Setup logger backend
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    // Set up RLTK
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    // Set up initial game state
    let mut gs = State { ecs: World::new() };

    // Add components
    // TODO: use init
    gs.ecs.register::<cmp::Pos>();
    gs.ecs.register::<cmp::Renderable>();
    gs.ecs.register::<cmp::Player>();
    gs.ecs.register::<cmp::Viewshed>();

    // Inizialie map
    let map =
        // Generate game map (only one for now)
        rnc::make_map(
            rnc::ConfigBuilder::default()
            .map_width(SCREEN_WIDTH)
            .map_height(SCREEN_HEIGHT)
            .build()
            .unwrap(),
        )
        .unwrap_or_else(|partial_map| {
            // TODO: use logging
            println!("Map generation was stopped prematurely...");

            partial_map
        });

    // TODO: TEST: Create player
    let player_spawn_point = map.idx_xy(
        *map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| t.is_passable())
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .expect("Could not pick player spawn point"),
    );
    gs.ecs
        .create_entity()
        .with(cmp::Player)
        .with(cmp::Pos::from(player_spawn_point))
        .with(cmp::Viewshed::new(10))
        .with(cmp::Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    // TODO: TEST: Create some entities
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(cmp::Pos { x: i * 7, y: 20 })
            .with(cmp::Viewshed::new(8))
            .with(cmp::Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    // Move map into world.
    gs.ecs.insert(map);

    // Game main loop
    rltk::main_loop(context, gs);
}
