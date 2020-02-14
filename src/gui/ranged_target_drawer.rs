extern crate rltk;

use specs::prelude::*;

use crate::{Context, get_screen_bounds, Map, State, Viewshed};

use self::rltk::{Algorithm2D, ColorPair, Point, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum RangedTargetResult { Cancel, NoResponse, Selected(Point) }

pub fn ranged_target<'ranged_target>(state: &'ranged_target mut State, context: &'ranged_target mut Context<'ranged_target>, settings: RangedTargetDrawerSettings) -> RangedTargetResult {
    RangedTargetDrawer {
        state,
        context,
        settings,
    }.draw_ranged_target()
}

struct RangedTargetDrawer<'a> {
    state: &'a mut State,
    context: &'a mut Context<'a>,
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

        let (screen_x, screen_y) = self.context.rltk.mouse_pos();
        let (min_x, _, min_y, _) = get_screen_bounds(&self.state.ecs, self.context);
        let (map_x, map_y) = (screen_x + min_x, screen_y + min_y);
        let is_in_range = in_range_tiles
            .iter()
            .any(|visible| visible.x == map_x && visible.y == map_y);

        if is_in_range {
            self.draw_radius(Point::new(screen_x, screen_y));
            self.context.ext_set_bg(Point::new(screen_x, screen_y), RGB::named(rltk::CYAN));
            if self.context.rltk.left_click {
                return RangedTargetResult::Selected(Point::new(map_x, map_y));
            }
        } else {
            self.context.ext_set_bg(Point::new(screen_x, screen_y), RGB::named(rltk::RED));
            if self.context.rltk.left_click {
                return RangedTargetResult::Cancel;
            }
        }

        RangedTargetResult::NoResponse
    }

    fn draw_range(&mut self) -> (Option<RangedTargetResult>, Vec<Point>) {
        let (min_x, _, min_y, _) = get_screen_bounds(&self.state.ecs, self.context);
        let player_entity = self.state.ecs.fetch::<Entity>();
        let player_position = self.state.ecs.fetch::<Point>();
        let viewsheds = self.state.ecs.read_storage::<Viewshed>();

        self.context.ext_print_color(Point::new(5, 0), "Select Target:", ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK)));

        let player_viewshed = viewsheds.get(*player_entity);
        let mut in_range_tiles = Vec::new();

        match player_viewshed {
            None => return (Some(RangedTargetResult::Cancel), in_range_tiles),
            Some(player_viewshed) => {
                for visible_tile in player_viewshed.visible_tiles.iter() {
                    let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_position, *visible_tile);
                    if distance <= self.settings.range as f32 {
                        self.context.ext_set_bg(Point::new(visible_tile.x - min_x, visible_tile.y - min_y), RGB::named(rltk::BLUE));
                        in_range_tiles.push(*visible_tile);
                    }
                }
            }
        }

        (None, in_range_tiles)
    }

    fn draw_radius(&mut self, screen_target: Point) {
        if self.settings.radius.is_none() {
            return;
        }
        let (min_x, _, min_y, _) = get_screen_bounds(&self.state.ecs, self.context);

        let map_target = Point::new(screen_target.x + min_x, screen_target.y + min_y);

        let map = self.state.ecs.fetch::<Map>();
        let blast_tiles = rltk::field_of_view(
            map_target,
            self.settings.radius.unwrap(),
            &*map);

        let valid_blast_tiles = blast_tiles
            .iter()
            .filter(|p| map.in_bounds(**p) && map.is_revealed(p.x, p.y));


        for tile in valid_blast_tiles {
            let screen_tile = Point::new(tile.x - min_x, tile.y - min_y);
            self.context.ext_set_bg(screen_tile, RGB::named(rltk::ORANGE));
        }
    }
}