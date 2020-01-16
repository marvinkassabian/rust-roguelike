#[macro_use]
extern crate specs_derive;

use rltk::{RGB, Rltk};
use specs::prelude::*;
use specs::WorldExt;

pub use components::*;
pub use map::*;
pub use player::*;
pub use random::*;
pub use rect::*;
pub use state::*;
pub use visibility_system::*;

rltk::add_wasm_support!();
mod map;
mod player;
mod rect;
mod components;
mod visibility_system;
mod state;
mod random;

fn main() {
    pub const WINDOW_WIDTH: i32 = 80;
    pub const WINDOW_HEIGHT: i32 = 40;

    let title = "Hello Rust World 2";
    const SHADER_PATH: &str = "resources";
    let context = Rltk::init_simple8x8(
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
        title,
        SHADER_PATH);
    let mut gs = State { ecs: World::new() };

    let map = new_map_rooms_and_corridors(WINDOW_WIDTH, WINDOW_HEIGHT);

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();

    let first_room = &(map.rooms).first();

    if let Some(first_room) = first_room {
        let pt = first_room.center();
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
            .build();
    }

    for room in map.rooms.iter().skip(1) {
        let pt = room.center();

        let mut rng = Random::new();

        let glyph = if rng.flip_coin() {
            rltk::to_cp437('o')
        } else {
            rltk::to_cp437('g')
        };

        gs.ecs
            .create_entity()
            .with(Position { x: pt.x, y: pt.y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs.insert(map);

    rltk::main_loop(context, gs);
}
