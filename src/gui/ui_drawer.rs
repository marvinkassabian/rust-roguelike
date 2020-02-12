extern crate rltk;

use specs::prelude::*;

use crate::{CombatStats, GameLog, MAP_HEIGHT, Player, TooltipDrawer, TooltipOrientation, WINDOW_HEIGHT, WINDOW_WIDTH};

use self::rltk::{Console, RGB, Rltk};

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    UiDrawer {
        ecs,
        ctx,
    }.draw_ui()
}

const HEALTH_TEXT_OFFSET: i32 = 12;
const HEALTH_BAR_START: i32 = 28;
const LOG_ENTRY_OFFSET: i32 = 2;

struct UiDrawer<'a> {
    ecs: &'a World,
    ctx: &'a mut Rltk,
}

impl<'a> UiDrawer<'a> {
    pub fn draw_ui(&mut self) {
        self.draw_game_log_frame();
        self.draw_health();
        self.draw_logs();
        self.draw_mouse_cursor();
        self.draw_tooltip();
    }

    fn draw_game_log_frame(&mut self) {
        self.ctx.draw_box(
            0,
            MAP_HEIGHT as i32,
            (WINDOW_WIDTH - 1) as i32,
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

        let mut y = WINDOW_HEIGHT - 2;
        for entry in log.entries.iter().skip(log.display_index as usize) {
            if y >= MAP_HEIGHT + 1 {
                self.ctx.print(LOG_ENTRY_OFFSET, y, &entry.get_formatted_message());
            } else {
                break;
            }

            y -= 1;
        }
    }

    fn draw_mouse_cursor(&mut self) {
        let (mouse_x, mouse_y) = self.ctx.mouse_pos();
        self.ctx.set_bg(mouse_x, mouse_y, RGB::named(rltk::MAGENTA));
    }

    fn draw_tooltip(&mut self) {
        let (mouse_x, mouse_y) = self.ctx.mouse_pos();

        TooltipDrawer {
            ecs: self.ecs,
            ctx: self.ctx,
        }.draw_tooltip(mouse_x, mouse_y, TooltipOrientation::Auto)
    }
}