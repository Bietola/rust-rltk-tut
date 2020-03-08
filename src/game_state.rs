use crate::components as cmp;
use crate::systems as sys;
use crate::map::{Map, Tile};
use rltk::{Console, GameState, Rltk};
use single::Single;
use specs::prelude::*;

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

    // Calculate new hypothetical player position
    let new_x = player_pos.x + delta_x;
    let new_y = player_pos.y + delta_y;

    // Don't move player onto walls
    let mp = world.fetch::<Map>();
    if mp.at(new_x, new_y) == Tile::Wall {
        return;
    }

    // Move him
    use std::cmp::{max, min};
    player_pos.x = max(0, min((mp.get_width() - 1) as i32, player_pos.x + delta_x));
    player_pos.y = max(0, min((mp.get_height() - 1) as i32, player_pos.y + delta_y));
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
        let mut vis = sys::Visibility;
        vis.run_now(&self.ecs);

        self.ecs.maintain();
    }

    // TODO: find a way to make this work
    //
    // Maybe use a macro, e.g.:
    //
    // ``` rust
    // let pl_viewshed = specs_get!{
    //     cmp::Player -> cmp::Viewshed
    // }
    // ```
    //
    // ``` rust
    // Get a generic `Component` belonging to the player
    // fn get_player_cmp<C>(&self) -> C
    //     where C: Component,
    // {
    //     let wanted_cmps = self.ecs.read_storage::<C>();
    //     let player = self.ecs.read_storage::<cmp::Player>();
    //     let (res, _pl) = (&wanted_cmps, &player)
    //         .join()
    //         .single()
    //         .unwrap_or_else(|e| panic!("Problem with findings player's {}: {}", std::any::type_name::<C>(), e));

    //     *res.clone()
    // }
    // ```
}

impl GameState for State {
    /// Simulate one tick of the game state
    /// NB. also redraws the screen
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear screen.
        ctx.cls();

        // Handle player input.
        player_input(self, ctx);

        // Run game systems.
        self.run_systems();

        // Draw map.
        let mp = self.ecs.fetch::<Map>();
        let viewsheds = self.ecs.read_storage::<cmp::Viewshed>();
        let player = self.ecs.read_storage::<cmp::Player>();
        let (pl_viewshed, _pl) = (&viewsheds, &player)
            .join()
            .single()
            // TODO: parameterize err msg.
            .unwrap_or_else(|e| panic!("Problem with findings player's viewshed: {}", e));
        mp.draw(pl_viewshed, ctx);

        // Draw entities.
        let positions = self.ecs.read_storage::<cmp::Pos>();
        let renderables = self.ecs.read_storage::<cmp::Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
