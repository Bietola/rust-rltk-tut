mod components;
mod consts;
mod game_state;
mod map;
mod utils;

use crate::components as cmp;
use crate::game_state::State;
use consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use map::gen::rnc;
use rltk::RGB;
use specs::prelude::*;
use log4rs;

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
    gs.ecs.register::<cmp::Pos>();
    gs.ecs.register::<cmp::Renderable>();
    gs.ecs.register::<cmp::Player>();
    // Generate game map (only one for now)
    gs.ecs.insert(
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
        })
    );

    // TODO: TEST: Create player
    gs.ecs
        .create_entity()
        .with(cmp::Player)
        .with(cmp::Pos { x: 10, y: 30 })
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
            .with(cmp::Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    // Game main loop
    rltk::main_loop(context, gs);
}
