extern crate rltk;
extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{Confusion, Map, Monster, Position, RNG, Viewshed, WantsToMelee, WantsToMove, WantsToTakeTurn, WantsToWait};

use self::rltk::Algorithm2D;

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, WantsToTakeTurn>,
        WriteStorage<'a, WantsToMove>,
        ReadStorage<'a, Viewshed>,
        WriteStorage<'a, WantsToWait>,
        ReadStorage<'a, Confusion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            entities,
            monster,
            positions,
            mut wants_to_melee,
            wants_to_take_turn,
            mut wants_to_move,
            viewsheds,
            mut wants_to_wait,
            confusions,
        ) = data;

        let map = &mut *map;
        let player_pos = *player_pos;
        let player_idx = map.point2d_to_index(player_pos);

        for (entity, _monster, position, _turn, viewshed) in (&entities, &monster, &positions, &wants_to_take_turn, &viewsheds).join() {
            if confusions.get(entity).is_some() {
                wants_to_wait.insert(entity, WantsToWait).expect("Unable to insert intent");
                continue;
            }

            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(position.x, position.y), player_pos);

            const IS_ADJACENT_DISTANCE: f32 = 1.01;

            if distance < IS_ADJACENT_DISTANCE {
                wants_to_melee.insert(entity, WantsToMelee { target: *player_entity }).expect("Unable to insert intent");
            } else if viewshed.visible_tiles.contains(&player_pos) {
                let monster_idx = map.xy_idx(position.x, position.y);
                let path = rltk::a_star_search(monster_idx, player_idx, map);

                const FIRST_STEP_INDEX: usize = 1;

                if path.success && path.steps.len() > FIRST_STEP_INDEX {
                    let first_step_idx = path.steps[FIRST_STEP_INDEX];
                    let first_step = map.index_to_point2d(first_step_idx);
                    wants_to_move.insert(entity, WantsToMove { destination: first_step }).expect("Unable to insert intent");
                } else {
                    wants_to_wait.insert(entity, WantsToWait).expect("Unable to insert intent");
                }
            } else {
                let delta: (i32, i32);

                match RNG.roll_die(4) {
                    1 => delta = (1, 0),
                    2 => delta = (-1, 0),
                    3 => delta = (0, 1),
                    4 => delta = (0, -1),
                    _ => delta = (0, 0),
                }

                let next_step = Point::new(position.x + delta.0, position.y + delta.1);

                if RNG.roll_die(7) > 1 {
                    wants_to_move.insert(entity, WantsToMove { destination: next_step }).expect("Unable to insert intent");
                } else {
                    wants_to_wait.insert(entity, WantsToWait).expect("Unable to insert intent");
                }
            }
        }
    }
}