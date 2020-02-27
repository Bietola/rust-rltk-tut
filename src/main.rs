mod components;
mod consts;
mod game_state;
mod map;

use crate::components as cmp;
use crate::game_state::State;
use rltk::RGB;
use specs::prelude::*;
use consts::{SCREEN_WIDTH, SCREEN_HEIGHT};

fn main() {
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
    gs.ecs.insert(map::gen::make_ugly_map(SCREEN_WIDTH, SCREEN_HEIGHT));

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
