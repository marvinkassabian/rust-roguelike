extern crate rltk;

use specs::prelude::*;

use crate::{CombatStats, GameLog, Map, MAP_HEIGHT, Name, Player, Position, WINDOW_HEIGHT, WINDOW_WIDTH};

use self::rltk::{Console, Point, RGB, Rltk};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    let mut drawer = UiDrawer {
        ecs,
        ctx,
    };

    drawer.draw_ui();
}

const HEALTH_TEXT_OFFSET: i32 = 12;
const HEALTH_BAR_START: i32 = 28;
const LOG_ENTRY_OFFSET: i32 = 2;

pub struct UiDrawer<'a> {
    ecs: &'a World,
    ctx: &'a mut Rltk,
}

impl<'a> UiDrawer<'a> {
    pub fn draw_ui(&mut self) {
        self.draw_gui_frame();
        self.draw_health();
        self.draw_logs();
        self.draw_mouse_cursor();
        self.draw_tooltip();
    }

    fn draw_gui_frame(&mut self) {
        self.ctx.draw_box(
            0,
            MAP_HEIGHT,
            WINDOW_WIDTH - 1,
            WINDOW_HEIGHT - MAP_HEIGHT - 1,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK));
    }

    fn draw_health(&mut self) {
        let combat_stats = self.ecs.read_storage::<CombatStats>();
        let players = self.ecs.read_storage::<Player>();

        for (_player, stats) in (&players, &combat_stats).join() {
            let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);

            self.ctx.print_color(
                HEALTH_TEXT_OFFSET,
                MAP_HEIGHT,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                &health);

            self.ctx.draw_bar_horizontal(
                HEALTH_BAR_START,
                MAP_HEIGHT,
                WINDOW_WIDTH - HEALTH_BAR_START - 1,
                stats.hp,
                stats.max_hp,
                RGB::named(rltk::RED),
                RGB::named(rltk::BLACK));
        }
    }

    fn draw_logs(&mut self) {
        let log = self.ecs.fetch::<GameLog>();

        let mut y = MAP_HEIGHT + 1;
        for entry in log.entries.iter() {
            if y < WINDOW_HEIGHT - 1 {
                self.ctx.print(LOG_ENTRY_OFFSET, y, entry);
            } else {
                break;
            }

            y += 1;
        }
    }

    fn draw_mouse_cursor(&mut self) {
        let (mouse_x, mouse_y) = self.ctx.mouse_pos();
        self.ctx.set_bg(mouse_x, mouse_y, RGB::named(rltk::MAGENTA));
    }

    fn draw_tooltip(&mut self) {
        let map = self.ecs.fetch::<Map>();
        let names = self.ecs.read_storage::<Name>();
        let positions = self.ecs.read_storage::<Position>();

        let (mouse_x, mouse_y) = self.ctx.mouse_pos();

        if !map.is_valid(mouse_x, mouse_y) {
            return;
        }

        let mut tooltip: Vec<String> = Vec::new();
        for (name, position) in (&names, &positions).join() {
            if position.x == mouse_x && position.y == mouse_y {
                tooltip.insert(0, name.name.to_string());
            }
        }

        if tooltip.is_empty() {
            return;
        }

        let max_width = tooltip.iter().map(|s| s.len()).max().unwrap() as i32;

        let width = max_width + 3;

        let arrow_pos: Point;
        let arrow_text: &str;
        let left_x: i32;
        let x_offset: i32;
        let x_direction: i32;

        let print_left_of_mouse = mouse_x > map.width / 2;

        if print_left_of_mouse {
            arrow_pos = Point::new(mouse_x - 2, mouse_y);
            arrow_text = "->";
            left_x = mouse_x - width;
            x_offset = 0;
            x_direction = -1;
        } else {
            arrow_pos = Point::new(mouse_x + 1, mouse_y);
            arrow_text = "<-";
            left_x = mouse_x + 3;
            x_offset = 1;
            x_direction = 1;
        }

        let mut y = mouse_y;
        for s in tooltip.iter() {
            self.ctx.print_color(
                left_x + x_offset,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                s);
            let padding = (width - s.len() as i32) - 1;
            for i in 0..padding {
                self.ctx.print_color(
                    arrow_pos.x + x_offset + x_direction * i,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    &" ".to_string());
            }
            y += 1;
        }

        self.ctx.print_color(
            arrow_pos.x,
            arrow_pos.y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::GREY),
            &arrow_text.to_string());
    }
}