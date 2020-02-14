use rltk::{ColorPair, Console, DrawBatch, Point, Rect, RGB, Rltk};

use crate::CONSOLE_INDEX;

const USE_BUFFER: bool = true;

pub trait RltkExt {
    fn ext_layered_set(&mut self, pos: Point, color: ColorPair, glyph: u8, height: usize, include_base: bool);

    fn ext_cls_all(&mut self);

    fn ext_cls(&mut self);

    fn ext_set_target(&mut self, index: usize);

    fn ext_draw_box(&mut self, pos: Rect, color: ColorPair);

    fn ext_set_bg(&mut self, pos: Point, bg: RGB);

    fn ext_print_color<S: ToString>(&mut self, pos: Point, text: S, color: ColorPair);

    fn ext_draw_bar_horizontal(&mut self, pos: Point, width: i32, n: i32, max: i32, color: ColorPair);

    fn ext_print<S: ToString>(&mut self, pos: Point, text: S);

    fn ext_set(&mut self, pos: Point, color: ColorPair, glyph: u8);
}

impl RltkExt for Rltk {
    fn ext_layered_set(&mut self, pos: Point, color: ColorPair, glyph: u8, height: usize, include_base: bool) {
        let all_layers = CONSOLE_INDEX.get_world_indices(include_base);
        let (layers, _) = all_layers.split_at(height);

        let total_layers = layers.len() as f32;
        let darkest_grey = RGB::named(rltk::GREY30);

        for layer in layers.iter() {
            let grey_ratio = 1. - (*layer as f32 / total_layers);
            let layer_grey = darkest_grey * grey_ratio;
            let shadow_fg = color.fg - layer_grey;
            self.ext_set(pos, ColorPair::new(shadow_fg, color.bg), glyph);
        }
    }

    fn ext_cls_all(&mut self) {
        let indices = CONSOLE_INDEX.get_all_indices();
        for index in indices {
            self.ext_set_target(index);
            self.ext_cls();
        }
    }

    fn ext_cls(&mut self) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.cls();
            draw_batch.submit(0);
        } else {
            self.cls();
        }
    }

    fn ext_set_target(&mut self, index: usize) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.target(index);
            draw_batch.submit(0);
        } else {
            self.set_active_console(index);
        }
    }

    fn ext_draw_box(&mut self, pos: Rect, color: ColorPair) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.draw_box(pos, color);
            draw_batch.submit(0);
        } else {
            self.draw_box(pos.x1, pos.y1, pos.width(), pos.height(), color.fg, color.bg);
        }
    }

    fn ext_set_bg(&mut self, pos: Point, bg: RGB) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.set_bg(pos, bg);
            draw_batch.submit(0);
        } else {
            self.set_bg(pos.x, pos.y, bg);
        }
    }

    fn ext_print_color<S: ToString>(&mut self, pos: Point, text: S, color: ColorPair) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.print_color(pos, text, color);
            draw_batch.submit(0);
        } else {
            self.print_color(pos.x, pos.y, color.fg, color.bg, &text.to_string());
        }
    }

    fn ext_draw_bar_horizontal(&mut self, pos: Point, width: i32, n: i32, max: i32, color: ColorPair) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.bar_horizontal(pos, width, n, max, color);
            draw_batch.submit(0);
        } else {
            self.draw_bar_horizontal(pos.x, pos.y, width, n, max, color.fg, color.bg);
        }
    }

    fn ext_print<S: ToString>(&mut self, pos: Point, text: S) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.print(pos, text);
            draw_batch.submit(0);
        } else {
            self.print(pos.x, pos.y, &text.to_string());
        }
    }

    fn ext_set(&mut self, pos: Point, color: ColorPair, glyph: u8) {
        if USE_BUFFER {
            let mut draw_batch = DrawBatch::new();
            draw_batch.set(pos, color, glyph);
            draw_batch.submit(0);
        } else {
            self.set(pos.x, pos.y, color.fg, color.bg, glyph);
        }
    }
}