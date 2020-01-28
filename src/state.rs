use rltk::{Console, console, GameState, Rltk};
use specs::prelude::*;
use specs::WorldExt;

use crate::{draw_map, Map, MonsterAI, player_input, Position, Renderable, VisibilitySystem};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    pub ecs: World,
    pub run_state: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        match self.run_state {
            RunState::Paused => {
                self.run_state = player_input(self, ctx);
            }
            RunState::Running => {
                self.run_systems();
                self.run_state = RunState::Paused;
            }
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            if map.is_visible(pos.x, pos.y) {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    pub fn run_systems(&mut self) {
        console::log("run_systems");

        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}
