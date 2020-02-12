#[macro_use]
extern crate specs_derive;

use specs::prelude::*;
use specs::WorldExt;

pub use components::*;
pub use context_builder::*;
pub use game_log::*;
pub use gui::*;
pub use map::*;
pub use player::*;
pub use random::*;
pub use rect::*;
pub use rltk_ext::*;
pub use spawner::*;
pub use state::*;
pub use systems::*;

rltk::add_wasm_support!();
mod systems;
mod map;
mod rltk_ext;
mod player;
mod rect;
mod components;
mod state;
mod random;
mod spawner;
mod gui;
mod game_log;
mod context_builder;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;
pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 50;

const TITLE: &str = "Goblin War Party";

fn main() {
    let mut gs = State { ecs: World::new() };
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog::new_with_first_log(format!("Welcome to {}", TITLE)));
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
    gs.ecs.register::<SuffersDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickUp>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDrop>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Confusion>();

    let map = new_map_rooms_and_corridors(&mut gs.ecs, MAP_WIDTH, MAP_HEIGHT);

    spawner::spawn_map(&mut gs.ecs, &map);

    gs.ecs.insert(map);

    let context = build_context(WINDOW_WIDTH, WINDOW_HEIGHT, TITLE);
    rltk::main_loop(context, gs);
}
