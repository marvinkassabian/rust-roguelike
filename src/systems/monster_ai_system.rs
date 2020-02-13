extern crate rltk;
extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{Map, Monster, Position, RunState, Viewshed, WantsToMelee};

use self::rltk::Algorithm2D;

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>, );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            run_state,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee) = data;

        if *run_state != RunState::MonsterTurn {
            return;
        }

        let map = &mut *map;
        let player_pos = *player_pos;
        let player_idx = map.point2d_to_index(player_pos);

        for (entity, mut viewshed, _monster, mut position) in (&entities, &mut viewshed, &monster, &mut position).join() {
            let point = Point::new(position.x, position.y);
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(point, player_pos);

            const IS_ADJACENT_DISTANCE: f32 = 1.5;

            if distance < IS_ADJACENT_DISTANCE {
                wants_to_melee.insert(entity, WantsToMelee {
                    target: *player_entity
                }).expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&player_pos) {
                let monster_idx = map.xy_idx(position.x, position.y);

                let path = rltk::a_star_search(monster_idx, player_idx, map);

                const FIRST_STEP_INDEX: usize = 1;

                if path.success && path.steps.len() > FIRST_STEP_INDEX {
                    let first_step_idx = path.steps[1];
                    let first_step = map.index_to_point2d(first_step_idx);
                    position.x = first_step.x;
                    position.y = first_step.y;
                    viewshed.dirty = true;
                    map.blocked[monster_idx as usize] = false;
                    map.blocked[first_step_idx as usize] = true;
                }
            }
        }
    }
}