use std::string::ToString;

use rltk::{Console, Rltk, SimpleConsole, SparseConsole};

use crate::{CONSOLE_INDEX, LAYER_COUNT, TITLE};

const LAYER_OFFSET_X: f32 = 0.06;
const LAYER_OFFSET_Y: f32 = 0.12;
const LAYER_STATIC_OFFSET_X: f32 = 0.;
const LAYER_STATIC_OFFSET_Y: f32 = 0.;
const SHADER_PATH: &str = "resources";
const PIXEL_WIDTH: u32 = 16;
const PIXEL_HEIGHT: u32 = 16;

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

impl<'a> ContextBuilder<'a> {
    pub fn create_context(&self) -> Rltk {
        let mut context = //Rltk::init_simple8x8(self.width, self.height, self.title, SHADER_PATH);
            Rltk::init_raw(self.width * PIXEL_WIDTH, self.height * PIXEL_HEIGHT, TITLE);

        let base_console_index = self.add_console(
            &mut context,
            AddConsoleParameter {
                has_bg: true,
                ..Default::default()
            },
        );

        check_console_index(CONSOLE_INDEX.base, base_console_index);

        for layer in 0..LAYER_COUNT {
            let offset_multiplier = (layer + 1) as f32;

            let layer_console_index = self.add_console(
                &mut context,
                AddConsoleParameter {
                    offset_x: LAYER_STATIC_OFFSET_X + LAYER_OFFSET_X * offset_multiplier,
                    offset_y: LAYER_STATIC_OFFSET_Y + LAYER_OFFSET_Y * offset_multiplier,
                    is_sparse: true,
                    ..Default::default()
                });

            check_console_index(CONSOLE_INDEX.layers[layer], layer_console_index);
        }

        let ui_console_index = self.add_console(
            &mut context,
            AddConsoleParameter {
                is_sparse: true,
                has_bg: true,
                ..Default::default()
            });

        check_console_index(CONSOLE_INDEX.ui, ui_console_index);

        context.set_active_console(CONSOLE_INDEX.base);
        context.with_post_scanlines(false);

        context
    }

    fn add_console(&self, context: &mut Rltk, params: AddConsoleParameter) -> usize {
        let font_path = format!("{}/terminal8x8.png", &SHADER_PATH.to_string());
        let font = context.register_font(rltk::Font::load(font_path, (PIXEL_WIDTH, PIXEL_HEIGHT)));

        let mut console: Box<dyn Console>;

        if params.is_sparse {
            console = SparseConsole::init(self.width, self.height, &context.backend);
        } else {
            console = SimpleConsole::init(self.width, self.height, &context.backend);
        }

        console.set_offset(params.offset_x, params.offset_y);
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

}

fn check_console_index(expected: usize, actual: usize) {
    if expected != actual {
        panic!("Incorrect console index: expected {}, got {}", expected, actual);
    }
}
