use std::fmt::Display;

use rltk::{ColorPair, Console, DrawBatch, Point, Rect, RGB, Rltk};

use crate::CONSOLE_INDEX;

//TODO swap bool with two interface implementations
const USE_BUFFER: bool = true;

pub struct Context<'a> {
    pub rltk: &'a mut Rltk
}

impl<'a> Context<'a> {
    pub fn new(rltk: &mut Rltk) -> Context {
        Context { rltk }
    }

    pub fn layered_set(&mut self, pos: Point, color: ColorPair, glyph: u8, height: usize, include_base: bool) {
        let all_layers = CONSOLE_INDEX.get_world_indices(include_base);
        let (layers, _) = all_layers.split_at(height);

        let total_layers = layers.len() as f32;
        let darkest_grey = RGB::named(rltk::GREY30);

        for (i, layer) in layers.iter().enumerate() {
            let grey_ratio = 1. - ((i as f32 + 1.) / total_layers);
            let layer_grey = darkest_grey * grey_ratio;
            let shadow_fg = color.fg - layer_grey;
            self.set_target(*layer);
            self.set(pos, ColorPair::new(shadow_fg, color.bg), glyph);
        }

        self.set_target(CONSOLE_INDEX.base);
    }

    pub fn cls_all(&mut self) {
        let indices = CONSOLE_INDEX.get_all_indices();
        for index in indices {
            self.set_target(index);
            self.cls();
        }

        self.set_target(CONSOLE_INDEX.base);
    }

    pub fn cls(&mut self) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.cls();
            draw_batch.submit(0);
        } else {
            self.rltk.cls();
        }
    }

    pub fn set_target(&mut self, index: usize) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.target(index);
            draw_batch.submit(0);
        } else {
            self.rltk.set_active_console(index);
        }
    }

    pub fn draw_box(&mut self, pos: Rect, color: ColorPair) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.draw_box(pos, color);
            draw_batch.submit(0);
        } else {
            self.rltk.draw_box(pos.x1, pos.y1, pos.width(), pos.height(), color.fg, color.bg);
        }
    }

    pub fn set_bg(&mut self, pos: Point, bg: RGB) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.set_bg(pos, bg);
            draw_batch.submit(0);
        } else {
            self.rltk.set_bg(pos.x, pos.y, bg);
        }
    }

    pub fn print_color<S: Display>(&mut self, pos: Point, text: S, color: ColorPair) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.print_color(pos, text, color);
            draw_batch.submit(0);
        } else {
            self.rltk.print_color(pos.x, pos.y, color.fg, color.bg, &text.to_string());
        }
    }

    pub fn draw_bar_horizontal(&mut self, pos: Point, width: i32, n: i32, max: i32, color: ColorPair) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.bar_horizontal(pos, width, n, max, color);
            draw_batch.submit(0);
        } else {
            self.rltk.draw_bar_horizontal(pos.x, pos.y, width, n, max, color.fg, color.bg);
        }
    }

    pub fn print<S: Display>(&mut self, pos: Point, text: S) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.print(pos, text);
            draw_batch.submit(0);
        } else {
            self.rltk.print(pos.x, pos.y, &text.to_string());
        }
    }

    pub fn set(&mut self, pos: Point, color: ColorPair, glyph: u8) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.set(pos, color, glyph);
            draw_batch.submit(0);
        } else {
            self.rltk.set(pos.x, pos.y, color.fg, color.bg, glyph);
        }
    }

    pub fn get_screen_size(&self) -> (u32, u32) {
        let (width, height) = self.rltk.get_char_size();
        (width, height)
    }

    pub fn print_color_centered<S: Display>(&mut self, y: i32, color: ColorPair, text: S) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.print_color_centered(y, text, color);
            draw_batch.submit(0);
        } else {
            self.rltk.print_color_centered(y, color.fg, color.bg, &text.to_string());
        }
    }
}