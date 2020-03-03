extern crate specs;

use rltk::{ColorPair, Point, RGB};
use specs::prelude::*;

use crate::{BlocksTile, CanMove, Map, ParticleBuilder, Player, Position, TakesTurn, Viewshed, WantsToMove};

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
        ReadStorage<'a, BlocksTile>,
        WriteExpect<'a, ParticleBuilder>,
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
            blocks_tiles,
            mut particle_builder,
        ) = data;

        for (entity, wants_to_move, mut takes_turn, mut position, can_move) in (&entities, &wants_to_move, &mut takes_turn, &mut positions, &can_move).join() {
            takes_turn.time_score += can_move.time_cost;

            let new_position = wants_to_move.destination;

            if map.is_blocked(new_position.x, new_position.y) {
                continue;
            }

            let old_position = Point::new(position.x, position.y);

            let old_position_idx = map.xy_idx(position.x, position.y);
            let new_position_idx = map.xy_idx(new_position.x, new_position.y);
            position.x = new_position.x;
            position.y = new_position.y;

            if let Some(viewshed) = viewsheds.get_mut(entity) {
                viewshed.dirty = true;
            }

            particle_builder.request(
                old_position,
                ColorPair::new(RGB::named(rltk::BLACK), RGB::named(rltk::WHITE_SMOKE)),
                rltk::to_cp437(' '),
                300.,
            );

            let is_blocker = blocks_tiles.get(entity).is_some();
            if is_blocker {
                map.blocked[old_position_idx as usize] = false;
                map.blocked[new_position_idx as usize] = true;
            }

            let is_player = players.get(entity).is_some();
            if is_player {
                player_position.x = new_position.x;
                player_position.y = new_position.y;
            }
        }

        wants_to_move.clear();
    }
}
