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
            if viewshed.visible_tiles.contains(&player_pos) {
                console::log(&format!("{} shouts at you", name.name));

                let monster_idx = map.xy_idx(position.x, position.y) as i32;

                let path = rltk::a_star_search(monster_idx, player_idx, map);

                const FIRST_STEP_INDEX: usize = 1;

                if path.success && path.steps.len() > FIRST_STEP_INDEX {
                    let first_step_idx = path.steps[1];
                    let first_step = map.index_to_point2d(first_step_idx);

                    position.x = first_step.x;
                    position.y = first_step.y;
                    viewshed.dirty = true;
                }
            }
        }
    }
}