use rltk::{Console, font, RGB, Rltk, SimpleConsole};

use crate::{TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};

const LAYER_OFFSET_X: f32 = 0.06;
const LAYER_OFFSET_Y: f32 = 0.12;
const LAYER_STATIC_OFFSET_X: f32 = -0.6;
const LAYER_STATIC_OFFSET_Y: f32 = -0.12;
const SHADER_PATH: &str = "resources";
const LAYER_COUNT: usize = 4;
pub static CONSOLE_INDEX: ConsoleIndex = ConsoleIndex { base: 0, layers: [1, 2, 3, 4], ui: 5 };

pub struct ConsoleIndex { pub base: usize, pub layers: [usize; LAYER_COUNT], pub ui: usize }

impl ConsoleIndex {
    pub fn get_all_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();

        indices.push(self.base);

        for layer in self.layers.iter() {
            indices.push(*layer);
        }

        indices.push(self.ui);

        indices
    }

    pub fn get_world_indices(&self, include_base: bool) -> Vec<usize> {
        let mut indices = Vec::new();

        if include_base {
            indices.push(self.base);
        }

        for layer in self.layers.iter() {
            indices.push(*layer);
        }

        indices
    }
}

pub trait RltkExt {
    fn add_sparse_console(&mut self, offset_x: f32, offset_y: f32, has_bg: bool) -> usize;

    fn layered_set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8, height: usize, include_base: bool);

    fn cls_all(&mut self);
}

pub fn init_context(width: u32, height: u32, title: &str) -> Rltk {
    let font_path = format!("{}/terminal8x8.png", &SHADER_PATH.to_string());
    let mut context = Rltk::init_raw(width * 8, height * 8, TITLE);
    let font = context.register_font(font::Font::load(&font_path.to_string(), (8, 8)));
    let base_console = SimpleConsole::init(width, height, &context.backend);
    let base_console_index = context.register_console(base_console, font);

    check_console_index(CONSOLE_INDEX.base, context.active_console);

    for layer in 0..LAYER_COUNT {
        let offset_multiplier = (layer + 1) as f32;
        let layer_console_index = context.add_sparse_console(
            LAYER_STATIC_OFFSET_X + LAYER_OFFSET_X * offset_multiplier,
            LAYER_STATIC_OFFSET_Y + LAYER_OFFSET_Y * offset_multiplier,
            false);

        check_console_index(CONSOLE_INDEX.layers[layer], layer_console_index);
    }

    let ui_console_index = context.add_sparse_console(0., 0., true);

    check_console_index(CONSOLE_INDEX.ui, ui_console_index);

    context.set_active_console(CONSOLE_INDEX.base);
    context.with_post_scanlines(false);

    context
}

fn check_console_index(expected: usize, actual: usize) {
    if expected != actual {
        panic!("Incorrect console index: expected {}, got {}", expected, actual);
    }
}

impl RltkExt for Rltk {
    fn add_sparse_console(&mut self, offset_x: f32, offset_y: f32, has_bg: bool) -> usize {
        let font_path = format!("{}/terminal8x8.png", &SHADER_PATH.to_string());
        let font = self.register_font(rltk::Font::load(font_path, (8, 8)));

        let mut sparse_console = rltk::SparseConsole::init(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, &self.backend);
        sparse_console.set_offset(offset_x, offset_y);
        if has_bg {
            self.register_console(sparse_console, font)
        } else {
            self.register_console_no_bg(sparse_console, font)
        }
    }

    fn layered_set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8, height: usize, include_base: bool) {
        let all_layers = CONSOLE_INDEX.get_world_indices(include_base);
        let (layers, _) = all_layers.split_at(height);

        let total_layers = layers.len() as f32;
        let darkest_grey = RGB::named(rltk::GREY40);

        for layer in layers.iter() {
            let grey_ratio = 1. - (*layer as f32 / total_layers);
            let layer_grey = darkest_grey * grey_ratio;
            let shadow_fg = fg - layer_grey;
            self.consoles[*layer].console.set(x, y, shadow_fg, bg, glyph);
        }
    }

    fn cls_all(&mut self) {
        let indices = CONSOLE_INDEX.get_all_indices();
        for index in indices {
            self.consoles[index].console.cls();
        }
    }
}