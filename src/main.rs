#[macro_use]
extern crate specs_derive;

use rltk::{Console, GameState, RGB, Rltk};
use specs::prelude::*;
use specs::WorldExt;

pub use map::*;
pub use player::*;
pub use rect::*;

rltk::add_wasm_support!();
mod map;
mod player;
pub mod rect;

pub const WINDOW_WIDTH: i32 = 80;
pub const WINDOW_HEIGHT: i32 = 40;

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

fn main() {
    let title = "Hello Rust World 2";
    const SHADER_PATH: &str = "resources";
    let context = Rltk::init_simple8x8(
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
        title,
        SHADER_PATH);
    let mut gs = State { ecs: World::new() };

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let last_room_or_none = rooms.last();
    if let Some(last_room) = last_room_or_none {
        let last_center = last_room.center();
        gs.ecs
            .create_entity()
            .with(Position { x: last_center.x, y: last_center.y })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .build();
    }

    rltk::main_loop(context, gs);
}
