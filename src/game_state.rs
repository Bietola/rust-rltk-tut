use crate::components as cmp;
use crate::consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use rltk::{Console, GameState, Rltk};
use single::Single;
use specs::prelude::*;
use crate::map;

/******************/
/* Helper methods */
/******************/
/// Moves player keeping him/her within the world bounds
fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    // Get player along with their position
    let positions = &mut world.write_storage::<cmp::Pos>();
    let players = &world.read_storage::<cmp::Player>();
    let (mut player_pos, _) = (positions, players)
        .join()
        .single()
        .expect("Trying to move unexistent player!");

    // Move him
    use std::cmp::{max, min};
    player_pos.x = max(0, min((SCREEN_WIDTH - 1) as i32, player_pos.x + delta_x));
    player_pos.y = max(0, min((SCREEN_HEIGHT - 1) as i32, player_pos.y + delta_y));
}

/// Handles player input
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    {
        use rltk::VirtualKeyCode::*;
        match ctx.key {
            None => {}
            Some(key) => match key {
                Left => try_move_player(-1, 0, &mut gs.ecs),
                Right => try_move_player(1, 0, &mut gs.ecs),
                Up => try_move_player(0, -1, &mut gs.ecs),
                Down => try_move_player(0, 1, &mut gs.ecs),
                _ => {}
            },
        }
    }
}

/************************/
/* Game state structure */
/************************/
pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        // TODO: add some systems
        self.ecs.maintain();
    }
}

impl GameState for State {
    /// Simulate one tick of the game state
    /// NB. also redraws the screen
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear screen
        ctx.cls();

        // Handle player input
        player_input(self, ctx);
       
        // Run game systems
        self.run_systems();

        // Draw map
        let mp = self.ecs.fetch::<map::Map>();
        map::draw_map(&mp, ctx);

        // Draw entities
        let positions = self.ecs.read_storage::<cmp::Pos>();
        let renderables = self.ecs.read_storage::<cmp::Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
