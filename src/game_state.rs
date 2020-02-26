use rltk::{Console, GameState, Rltk};
use specs::prelude::*;
use crate::components as cmp;

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    /// Simulate one tick of the game state
    /// NB. also redraws the screen
    fn tick(&mut self, ctx: &mut Rltk) {
        let positions = self.ecs.read_storage::<cmp::Pos>();
        let renderables = self.ecs.read_storage::<cmp::Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

