extern crate rltk;
extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{Confusion, console_log, Map, Monster, Name, Position, RNG, Viewshed, WaitCause, WantsToMelee, WantsToMove, WantsToTakeTurn, WantsToWait};

use self::rltk::Algorithm2D;

pub struct MonsterAI;

impl MonsterAI {
    pub const NAME: &'static str = "mob";
}

#[derive(Debug)]
enum MonsterTurnAction {
    Melee(Entity),
    Move(Point),
    Wait { is_confused: bool },
}

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
        ReadStorage<'a, Name>,
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
            names,
        ) = data;

        let map = &mut *map;
        let player_pos = *player_pos;
        let player_idx = map.point2d_to_index(player_pos);

        let get_action = |entity: Entity, position: &Position, viewshed: &Viewshed| -> MonsterTurnAction {
            if confusions.get(entity).is_some() {
                return MonsterTurnAction::Wait { is_confused: true };
            }

            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(position.x, position.y), player_pos);

            const IS_ADJACENT_DISTANCE: f32 = 1.01;

            if distance < IS_ADJACENT_DISTANCE {
                return MonsterTurnAction::Melee(*player_entity);
            } else if viewshed.visible_tiles.contains(&player_pos) {
                let monster_idx = map.xy_idx(position.x, position.y);
                let path = rltk::a_star_search(monster_idx, player_idx, map);

                const FIRST_STEP_INDEX: usize = 1;

                if path.success && path.steps.len() > FIRST_STEP_INDEX {
                    let first_step_idx = path.steps[FIRST_STEP_INDEX];
                    let first_step = map.index_to_point2d(first_step_idx);
                    return MonsterTurnAction::Move(first_step);
                } else {
                    return MonsterTurnAction::Wait { is_confused: false };
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

                let (delta_x, delta_y) = delta;

                let next_step = Point::new(position.x + delta_x, position.y + delta_y);

                if RNG.roll_die(7) > 1 {
                    return MonsterTurnAction::Move(next_step);
                } else {
                    return MonsterTurnAction::Wait { is_confused: false };
                }
            }
        };

        for (entity, _monster, position, _turn, viewshed, name) in (&entities, &monster, &positions, &wants_to_take_turn, &viewsheds, &names).join() {
            let action = get_action(entity, position, viewshed);

            console_log(format!("           {}: {:?}", name.name, action));

            match action {
                MonsterTurnAction::Melee(target) => {
                    wants_to_melee.insert(entity, WantsToMelee { target }).expect("Unable to insert intent");
                }
                MonsterTurnAction::Move(destination) => {
                    wants_to_move.insert(entity, WantsToMove { destination }).expect("Unable to insert intent");
                }
                MonsterTurnAction::Wait { is_confused } => {
                    let cause = match is_confused {
                        true => WaitCause::Confusion,
                        false => WaitCause::Choice,
                    };

                    wants_to_wait.insert(entity, WantsToWait { cause }).expect("Unable to insert intent");
                }
            }
        }
    }
}