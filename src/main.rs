#[macro_use]
extern crate specs_derive;

use rltk::Rltk;
use specs::prelude::*;
use specs::WorldExt;

pub use components::*;
pub use game_log::*;
pub use gui::*;
pub use map::*;
pub use player::*;
pub use random::*;
pub use rect::*;
pub use spawner::*;
pub use state::*;
pub use systems::*;

rltk::add_wasm_support!();
mod systems;
mod map;
mod player;
mod rect;
mod components;
mod state;
mod random;
mod spawner;
mod gui;
mod game_log;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 50;
pub const MAP_HEIGHT: i32 = 43;

const SHADER_PATH: &str = "resources";
const TITLE: &str = "Goblin War Party";

fn main() {
    let mut context = Rltk::init_simple8x8(
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
        TITLE,
        SHADER_PATH);
    context.with_post_scanlines(true);

    let mut gs = State { ecs: World::new() };
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog { entries: vec![format!("Welcome to {}", TITLE)] });
    gs.ecs.insert(Random::new());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();

    let map = new_map_rooms_and_corridors(&mut gs.ecs, WINDOW_WIDTH, MAP_HEIGHT);

    spawner::spawn_map(&mut gs.ecs, &map);

    gs.ecs.insert(map);

    rltk::main_loop(context, gs);
}
