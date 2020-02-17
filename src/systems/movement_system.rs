extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{CanMove, Map, Player, Position, TakesTurn, Viewshed, WantsToMove};

pub struct MovementSystem;

impl MovementSystem {
    pub const NAME: &'static str = "move";
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, TakesTurn>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, Point>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, CanMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut wants_to_move,
            mut takes_turn,
            mut positions,
            mut viewsheds,
            entities,
            players,
            mut player_position,
            mut map,
            can_move,
        ) = data;

        for (entity, wants_to_move, mut takes_turn, mut position, mut viewshed, can_move) in (&entities, &wants_to_move, &mut takes_turn, &mut positions, &mut viewsheds, &can_move).join() {
            takes_turn.time_score += can_move.speed;

            let new_position = wants_to_move.destination;

            let is_player = players.get(entity).is_some();

            if map.is_blocked(new_position.x, new_position.y) {
                continue;
            }

            let old_position_idx = map.xy_idx(position.x, position.y);
            let new_position_idx = map.xy_idx(new_position.x, new_position.y);
            position.x = new_position.x;
            position.y = new_position.y;
            viewshed.dirty = true;
            map.blocked[old_position_idx as usize] = false;

            if !is_player {
                map.blocked[new_position_idx as usize] = true;
            } else {
                player_position.x = new_position.x;
                player_position.y = new_position.y;
            }
        }

        wants_to_move.clear();
    }
}
