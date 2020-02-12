extern crate rltk;

use specs::prelude::*;

use crate::{Map, State, Viewshed};

use self::rltk::{Algorithm2D, Console, Point, RGB, Rltk};

#[derive(PartialEq, Copy, Clone)]
pub enum RangedTargetResult { Cancel, NoResponse, Selected(Point) }

pub fn ranged_target(state: &mut State, context: &mut Rltk, settings: RangedTargetDrawerSettings) -> RangedTargetResult {
    RangedTargetDrawer {
        state,
        context,
        settings,
    }.draw_ranged_target()
}

struct RangedTargetDrawer<'a> {
    state: &'a mut State,
    context: &'a mut Rltk,
    settings: RangedTargetDrawerSettings,
}

pub struct RangedTargetDrawerSettings {
    pub range: i32,
    pub radius: Option<i32>,
}

impl<'a> RangedTargetDrawer<'a> {
    pub fn draw_ranged_target(&mut self) -> RangedTargetResult {
        let (result_or_none, in_range_tiles) = self.draw_range();

        if result_or_none.is_some() {
            return result_or_none.unwrap();
        }

        let (mouse_x, mouse_y) = self.context.mouse_pos();
        let is_in_range = in_range_tiles
            .iter()
            .any(|visible| visible.x == mouse_x && visible.y == mouse_y);

        if is_in_range {
            self.draw_radius(Point::new(mouse_x, mouse_y));
            self.context.set_bg(mouse_x, mouse_y, RGB::named(rltk::CYAN));
            if self.context.left_click {
                return RangedTargetResult::Selected(Point::new(mouse_x, mouse_y));
            }
        } else {
            self.context.set_bg(mouse_x, mouse_y, RGB::named(rltk::RED));
            if self.context.left_click {
                return RangedTargetResult::Cancel;
            }
        }

        RangedTargetResult::NoResponse
    }

    fn draw_range(&mut self) -> (Option<RangedTargetResult>, Vec<Point>) {
        let player_entity = self.state.ecs.fetch::<Entity>();
        let player_position = self.state.ecs.fetch::<Point>();
        let viewsheds = self.state.ecs.read_storage::<Viewshed>();

        self.context.print_color(5, 0, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Select Target:");

        let player_viewshed = viewsheds.get(*player_entity);
        let mut in_range_tiles = Vec::new();

        match player_viewshed {
            None => return (Some(RangedTargetResult::Cancel), in_range_tiles),
            Some(player_viewshed) => {
                for visible_tile in player_viewshed.visible_tiles.iter() {
                    let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_position, *visible_tile);
                    if distance <= self.settings.range as f32 {
                        self.context.set_bg(visible_tile.x, visible_tile.y, RGB::named(rltk::BLUE));
                        in_range_tiles.push(*visible_tile);
                    }
                }
            }
        }

        (None, in_range_tiles)
    }

    fn draw_radius(&mut self, target: Point) {
        if self.settings.radius.is_none() {
            return;
        }

        let map = self.state.ecs.fetch::<Map>();
        let blast_tiles = rltk::field_of_view(
            target,
            self.settings.radius.unwrap(),
            &*map);

        let valid_blast_tiles = blast_tiles
            .iter()
            .filter(|p| map.in_bounds(**p) && map.is_revealed(p.x, p.y));

        for tile in valid_blast_tiles {
            self.context.set_bg(tile.x, tile.y, RGB::named(rltk::ORANGE));
        }
    }
}