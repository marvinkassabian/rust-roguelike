#[macro_use]
extern crate specs_derive;

use specs::prelude::*;
use specs::WorldExt;

pub use components::*;
pub use context::*;
pub use game_log::*;
pub use gui::*;
pub use map::*;
pub use player::*;
pub use random::*;
pub use spawner::*;
pub use state::*;
pub use systems::*;

rltk::add_wasm_support!();
mod systems;
mod playground;
mod map;
mod player;
mod components;
mod state;
mod random;
mod spawner;
mod gui;
mod game_log;
mod context;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;
pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 50;

const TITLE: &str = "Goblin War Party";

fn main() {
    let mut state = State { ecs: World::new(), systems: SysRunner::new() };
    state.ecs.insert(RunState::PreRun);
    state.ecs.insert(GameLog::new_with_first_log(format!("Welcome to {}", TITLE)));
    state.ecs.insert(Random::new());

    state.ecs.register::<Position>();
    state.ecs.register::<Renderable>();
    state.ecs.register::<Player>();
    state.ecs.register::<Viewshed>();
    state.ecs.register::<Monster>();
    state.ecs.register::<Name>();
    state.ecs.register::<BlocksTile>();
    state.ecs.register::<CombatStats>();
    state.ecs.register::<WantsToMelee>();
    state.ecs.register::<SuffersDamage>();
    state.ecs.register::<Item>();
    state.ecs.register::<InBackpack>();
    state.ecs.register::<WantsToPickUp>();
    state.ecs.register::<WantsToUseItem>();
    state.ecs.register::<WantsToDrop>();
    state.ecs.register::<Consumable>();
    state.ecs.register::<ProvidesHealing>();
    state.ecs.register::<Ranged>();
    state.ecs.register::<InflictsDamage>();
    state.ecs.register::<AreaOfEffect>();
    state.ecs.register::<Confusion>();

    let map = new_map_rooms_and_corridors(&mut state.ecs, MAP_WIDTH, MAP_HEIGHT);

    spawner::spawn_map(&mut state.ecs, &map);

    state.ecs.insert(map);

    let context = build_context(WINDOW_WIDTH, WINDOW_HEIGHT, TITLE);
    rltk::main_loop(context, state);
}
