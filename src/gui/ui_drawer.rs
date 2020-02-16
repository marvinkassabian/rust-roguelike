extern crate rltk;

use specs::prelude::*;

use crate::{CombatStats, Context, GameLog, Player, TooltipDrawer, TooltipOrientation};

use self::rltk::{ColorPair, Point, Rect, RGB};

pub const GAME_LOG_HEIGHT: i32 = 7;
const HEALTH_TEXT_OFFSET: i32 = 12;
const HEALTH_BAR_START: i32 = 28;
const LOG_ENTRY_OFFSET: i32 = 2;

pub fn draw_ui(ecs: &World, context: &mut Context) {
    let (window_width, window_height) = context.get_screen_size();

    UiDrawer {
        ecs,
        context,
        dimensions: Dimensions {
            map_screen_height: window_height as i32 - GAME_LOG_HEIGHT,
            window_width: window_width as i32,
            window_height: window_height as i32,
        },
    }.draw_ui()
}

struct UiDrawer<'a, 'b> {
    pub ecs: &'a World,
    pub context: &'a mut Context<'b>,
    pub dimensions: Dimensions,
}

struct Dimensions {
    pub map_screen_height: i32,
    pub window_width: i32,
    pub window_height: i32,
}

impl<'a, 'b> UiDrawer<'a, 'b> {
    pub fn draw_ui(&mut self) {
        self.draw_game_log_frame();
        self.draw_health();
        self.draw_logs();
        self.draw_mouse_cursor();
        self.draw_tooltip();
    }

    fn draw_game_log_frame(&mut self) {
        self.context.draw_box(
            Rect::with_size(
                0,
                self.dimensions.map_screen_height as i32,
                (self.dimensions.window_width - 1) as i32,
                GAME_LOG_HEIGHT - 1),
            ColorPair::new(
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK)));
    }

    fn draw_health(&mut self) {
        let combat_stats = self.ecs.read_storage::<CombatStats>();
        let players = self.ecs.read_storage::<Player>();

        for (_player, stats) in (&players, &combat_stats).join() {
            let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);

            self.context.print_color(
                Point::new(
                    HEALTH_TEXT_OFFSET,
                    self.dimensions.map_screen_height),
                &health,
                ColorPair::new(
                    RGB::named(rltk::YELLOW),
                    RGB::named(rltk::BLACK)),
            );

            self.context.draw_bar_horizontal(
                Point::new(
                    HEALTH_BAR_START,
                    self.dimensions.map_screen_height),
                self.dimensions.window_width - HEALTH_BAR_START - 1,
                stats.hp,
                stats.max_hp,
                ColorPair::new(
                    RGB::named(rltk::RED),
                    RGB::named(rltk::BLACK)));
        }
    }

    fn draw_logs(&mut self) {
        let log = self.ecs.fetch::<GameLog>();

        let mut y = self.dimensions.window_height - 2;
        for entry in log.entries.iter().skip(log.display_index as usize) {
            if y >= self.dimensions.window_height - GAME_LOG_HEIGHT + 1 {
                self.context.print(Point::new(LOG_ENTRY_OFFSET, y), &entry.get_formatted_message());
            } else {
                break;
            }

            y -= 1;
        }
    }

    fn draw_mouse_cursor(&mut self) {
        let (mouse_x, mouse_y) = self.context.rltk.mouse_pos();
        self.context.set_bg(Point::new(mouse_x, mouse_y), RGB::named(rltk::MAGENTA));
    }

    fn draw_tooltip(&mut self) {
        let (mouse_x, mouse_y) = self.context.rltk.mouse_pos();

        TooltipDrawer { ecs: self.ecs, context: self.context }
            .draw_tooltip(mouse_x, mouse_y, TooltipOrientation::Auto);
    }
}