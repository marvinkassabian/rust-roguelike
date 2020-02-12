use rltk::{RGB, Rltk};

pub const LAYER_COUNT: usize = 4;
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
    fn layered_set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8, height: usize, include_base: bool);

    fn cls_all(&mut self);
}

impl RltkExt for Rltk {
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