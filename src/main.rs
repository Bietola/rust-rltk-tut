mod components;
mod game_state;

use rltk::RGB;
use crate::components as cmp;
use crate::game_state::State;
use specs::prelude::*;

fn main() {
    // Set up RLTK
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    // Set up initial game state
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<cmp::Pos>();
    gs.ecs.register::<cmp::Renderable>();

    // Create some entities
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
