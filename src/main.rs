rltk::add_wasm_support!();
use rltk::{Console, GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::{Component, World};
use std::cmp::{max, min};
#[macro_use]
extern crate specs_derive;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World 2");
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

fn main() {
    let width = 80;
    let height = 50;
    let title = "Hello Rust World 2";
    let shader_path = "resources";
    let context = Rltk::init_simple8x8(width, height, title, shader_path);
    let gs = State {};
    rltk::main_loop(context, gs);
}
