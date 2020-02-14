extern crate rltk;

use std::string::ToString;

use specs::prelude::*;

use crate::{CameraRenderer, CONSOLE_INDEX, Map, Name, Position, RltkExt};

use self::rltk::{ColorPair, Point, RGB, Rltk};

pub struct TooltipDrawer<'a> {
    pub ecs: &'a World,
    pub context: &'a mut Rltk,
}

pub enum TooltipOrientation { Left, Right, Auto }

impl<'a> TooltipDrawer<'a> {
    pub fn draw_tooltip(&mut self, screen_x: i32, screen_y: i32, orientation: TooltipOrientation) {
        self.context.ext_set_target(CONSOLE_INDEX.ui);

        self.draw_tooltip_internal(screen_x, screen_y, orientation);

        self.context.ext_set_target(CONSOLE_INDEX.base);
    }

    fn draw_tooltip_internal(&mut self, screen_x: i32, screen_y: i32, orientation: TooltipOrientation) {
        let (min_x, _, min_y, _) = CameraRenderer { ecs: self.ecs, context: self.context }.get_screen_bounds();
        let map_x = screen_x + min_x;
        let map_y = screen_y + min_y;
        let map = self.ecs.fetch::<Map>();
        let names = self.ecs.read_storage::<Name>();
        let positions = self.ecs.read_storage::<Position>();

        if !map.is_valid(map_x, map_y) || !map.is_visible(map_x, map_y) {
            return;
        }

        let mut tooltip: Vec<String> = Vec::new();
        for (name, position) in (&names, &positions).join() {
            if position.x == map_x && position.y == map_y {
                tooltip.insert(0, name.name.to_string());
            }
        }

        if tooltip.is_empty() {
            return;
        }

        let max_width = tooltip.iter().map(|s| s.len()).max().unwrap() as i32;

        let fg: RGB = RGB::named(rltk::WHITE);
        let bg: RGB = RGB::named(rltk::DARK_GREY);

        let width = max_width;

        let arrow_pos: Point;
        let arrow_text: &str = "-";
        let arrow_length = arrow_text.len() as i32;
        let left_x: i32;

        let print_left_of_mouse = match orientation {
            TooltipOrientation::Left => true,
            TooltipOrientation::Right => false,
            TooltipOrientation::Auto => screen_x > map.width / 2,
        };

        if print_left_of_mouse {
            arrow_pos = Point::new(screen_x - 1, screen_y);
            left_x = screen_x - width - arrow_length;
        } else {
            arrow_pos = Point::new(screen_x + 1, screen_y);
            left_x = screen_x + 2;
        }

        let mut y = screen_y;
        for entity_name in tooltip.iter() {
            self.context.ext_print_color(Point::new(left_x, y), entity_name, ColorPair::new(fg, bg));
            let name_length = entity_name.len() as i32;
            let padding = width - name_length as i32;

            for i in 0..padding {
                self.context.ext_print_color(Point::new(left_x + name_length + i, y), &" ".to_string(), ColorPair::new(fg, bg));
            }
            y += 1;
        }

        self.context.ext_print_color(Point::new(arrow_pos.x, arrow_pos.y), &arrow_text.to_string(), ColorPair::new(fg, bg));
    }
}