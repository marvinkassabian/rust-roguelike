extern crate rltk;
extern crate specs;

use rltk::{console, Point};
use specs::prelude::*;

use crate::{Map, Monster, Name, Position, Viewshed};

use self::rltk::Algorithm2D;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            mut viewshed,
            monster,
            name,
            mut position) = data;

        let map = &mut *map;
        let player_pos = *player_pos;
        let player_idx = map.point2d_to_index(player_pos);

        for (mut viewshed, _monster, name, mut position) in (&mut viewshed, &monster, &name, &mut position).join() {
            let point = Point::new(position.x, position.y);
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(point, player_pos);

            if distance < 1.5 {
                console::log(&format!("{} shouts at you", name.name));
            }

            if viewshed.visible_tiles.contains(&player_pos) {
                console::log(&format!("{} sees the player", name.name));
                let monster_idx = map.xy_idx(position.x, position.y) as i32;

                let path = rltk::a_star_search(monster_idx, player_idx, map);

                const FIRST_STEP_INDEX: usize = 1;

                if path.success && path.steps.len() > FIRST_STEP_INDEX {
                    console::log(&format!("{} has a path to the player", name.name));

                    let first_step_idx = path.steps[1];
                    let first_step = map.index_to_point2d(first_step_idx);

                    if first_step_idx == player_idx {
                        console::log(&format!("{} is blocked by player", name.name));
                    } else if !map.is_blocked(first_step.x, first_step.y) {
                        console::log("Path:");
                        for (i, step_idx) in path.steps.iter().enumerate() {
                            let step = map.index_to_point2d(*step_idx);
                            console::log(&format!("{} ({}, {})", i, step.x, step.y));
                        }

                        position.x = first_step.x;
                        position.y = first_step.y;
                        viewshed.dirty = true;
                        map.blocked[monster_idx as usize] = false;
                        map.blocked[first_step_idx as usize] = true;
                    } else {
                        console::log(&format!("{} is blocked", name.name));
                    }
                }
            }
        }
    }
}