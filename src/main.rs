#[macro_use]
extern crate specs_derive;

use rltk::{Point, RGB, Rltk};
use specs::prelude::*;
use specs::WorldExt;

pub use components::*;
pub use map::*;
pub use player::*;
pub use random::*;
pub use rect::*;
pub use state::*;
pub use systems::*;

rltk::add_wasm_support!();
pub mod systems;
mod map;
mod player;
mod rect;
mod components;
mod state;
mod random;

fn main() {
    pub const WINDOW_WIDTH: i32 = 80;
    pub const WINDOW_HEIGHT: i32 = 50;

    let title = "Hello Rust World 2";
    const SHADER_PATH: &str = "resources";
    let mut context = Rltk::init_simple8x8(
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
        title,
        SHADER_PATH);
    context.with_post_scanlines(true);

    let mut gs = State { ecs: World::new(), run_state: RunState::Running };

    let map = new_map_rooms_and_corridors(WINDOW_WIDTH, WINDOW_HEIGHT);

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

    let first_room = &(map.rooms).first();
    let mut rng = Random::new();

    if let Some(first_room) = first_room {
        let pt = first_room.center();
        gs.ecs.insert(Point::new(pt.x, pt.y));

        gs.ecs
            .create_entity()
            .with(Position { x: pt.x, y: pt.y })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Name { name: "Player".to_string() })
            .with(CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            })
            .build();
    }

    for (i, room) in map.rooms.iter().enumerate().skip(1) {
        let pt = room.center();

        let glyph;
        let name;

        if rng.flip_coin() {
            glyph = rltk::to_cp437('o');
            name = "Orc";
        } else {
            glyph = rltk::to_cp437('g');
            name = "Goblin";
        };

        gs.ecs
            .create_entity()
            .with(Position { x: pt.x, y: pt.y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Monster {})
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Name { name: format!("{} #{}", name, i).to_string() })
            .with(BlocksTile {})
            .with(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
            .build();
    }

    gs.ecs.insert(map);

    rltk::main_loop(context, gs);
}
