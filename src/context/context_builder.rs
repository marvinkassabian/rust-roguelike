use std::string::ToString;

use rltk::{Console, console, Rltk, RltkBuilder, SimpleConsole, SparseConsole};

use crate::{CONSOLE_INDEX, LAYER_COUNT, TITLE};

const LAYER_OFFSET_X: f32 = 0.15;
const LAYER_OFFSET_Y: f32 = 0.28;
const LAYER_STATIC_OFFSET_X: f32 = 0.;
const LAYER_STATIC_OFFSET_Y: f32 = 0.;
const SHADER_PATH: &str = "resources";
const TILE_WIDTH: u32 = 8 * SCREEN_MULTIPLIER;
const TILE_HEIGHT: u32 = 8 * SCREEN_MULTIPLIER;

const SCREEN_MULTIPLIER: u32 = 1;
const SCALE_STEP: u32 = 2;

pub struct ContextBuilder<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
}

pub fn build_context(width: i32, height: i32, title: &str) -> Rltk {
    ContextBuilder {
        width: width as u32,
        height: height as u32,
        title,
    }.create_context()
}

const USE_DEPTH: bool = false;

impl<'a> ContextBuilder<'a> {
    pub fn create_context(&self) -> Rltk {
        let mut context = RltkBuilder::simple(self.width, self.height)
            .with_title(TITLE).build();

        let mut tile_width = TILE_WIDTH;
        let mut tile_height = TILE_HEIGHT;

        let base_console_index = self.add_console(
            &mut context,
            AddConsoleParameter {
                has_bg: true,
                tile_width: Some(tile_width),
                tile_height: Some(tile_height),
                ..Default::default()
            },
        );

        if USE_DEPTH {
            tile_width += SCALE_STEP;
            tile_height += SCALE_STEP;
        }

        check_console_index(CONSOLE_INDEX.base, base_console_index);

        for layer in 0..LAYER_COUNT {
            let offset_multiplier = (layer + 1) as f32;

            let layer_console_index = self.add_console(
                &mut context,
                AddConsoleParameter {
                    offset_x: LAYER_STATIC_OFFSET_X + LAYER_OFFSET_X * offset_multiplier,
                    offset_y: LAYER_STATIC_OFFSET_Y + LAYER_OFFSET_Y * offset_multiplier,

                    tile_width: Some(tile_width),
                    tile_height: Some(tile_height),
                    is_sparse: true,
                    ..Default::default()
                });

            if USE_DEPTH {
                tile_width += SCALE_STEP;
                tile_height += SCALE_STEP;
            }

            check_console_index(CONSOLE_INDEX.layers[layer], layer_console_index);
        }

        let ui_console_index = self.add_console(
            &mut context,
            AddConsoleParameter {
                is_sparse: true,
                has_bg: true,

                tile_width: Some(tile_width),
                tile_height: Some(tile_height),
                ..Default::default()
            });

        check_console_index(CONSOLE_INDEX.ui, ui_console_index);

        context.set_active_console(CONSOLE_INDEX.base);

        context.with_post_scanlines(false);

        context
    }

    fn add_console(&self, context: &mut Rltk, params: AddConsoleParameter) -> usize {
        let tile_width = params.tile_width.unwrap_or(TILE_WIDTH);
        let tile_height = params.tile_width.unwrap_or(TILE_HEIGHT);
        let width_ratio = tile_width as f32 / TILE_WIDTH as f32;
        let height_ratio = tile_height as f32 / TILE_HEIGHT as f32;

        console::log(format!("tile_width: {}, tile_height: {}", tile_width, tile_height));

        let font_path = format!("{}/terminal8x8.png", &SHADER_PATH.to_string());
        let font = context.register_font(rltk::Font::load(font_path, (tile_width, tile_height)));

        let mut console: Box<dyn Console>;

        let width = (self.width as f32 * width_ratio) as u32;
        let height = (self.height as f32 * height_ratio) as u32;

        if params.is_sparse {
            console = SparseConsole::init(width, height, &context.backend);
        } else {
            console = SimpleConsole::init(width, height, &context.backend);
        }

        let mut offset_x = params.offset_x;
        let mut offset_y = params.offset_y;

        if USE_DEPTH {
            offset_x += ((width - self.width) / 2) as f32;
            offset_y -= ((height - self.height) / 2) as f32;
        }

        console::log(format!("offset_x: {}, offset_y: {}", offset_x, offset_y));

        console.set_offset(offset_x, offset_y);
        if params.has_bg {
            context.register_console(console, font)
        } else {
            context.register_console_no_bg(console, font)
        }
    }
}

#[derive(Default)]
struct AddConsoleParameter {
    pub offset_x: f32,
    pub offset_y: f32,
    pub is_sparse: bool,
    pub has_bg: bool,
    pub tile_width: Option<u32>,
    pub tile_height: Option<u32>,

}

fn check_console_index(expected: usize, actual: usize) {
    if expected != actual {
        panic!("Incorrect console index: expected {}, got {}", expected, actual);
    }
}
