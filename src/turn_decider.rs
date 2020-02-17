extern crate rltk;
extern crate specs;

use specs::prelude::*;

use crate::{GlobalTurn, IsVisible, Name, Player, RunState, TakesTurn, WantsToTakeTurn};

pub struct TurnDecider<'a> {
    pub ecs: &'a World,
}

pub fn decide_turn(ecs: &mut World) -> RunState {
    TurnDecider { ecs }.decide_turn()
}

impl<'a> TurnDecider<'a> {
    pub fn decide_turn(&mut self) -> RunState {
        let (
            mut takes_turn,
            mut wants_to_take_turn,
            global_turn,
            player,
            entities,
            names,
            is_visible,
        ) = (
            self.ecs.write_storage::<TakesTurn>(),
            self.ecs.write_storage::<WantsToTakeTurn>(),
            self.ecs.read_storage::<GlobalTurn>(),
            self.ecs.read_storage::<Player>(),
            self.ecs.entities(),
            self.ecs.read_storage::<Name>(),
            self.ecs.read_storage::<IsVisible>(),
        );

        wants_to_take_turn.clear();

        let mut data = (&entities, &mut takes_turn, &names).join().collect::<Vec<_>>();
        data.sort_by(|a, b| {
            let (_, a_takes_turn, _) = a;
            let (_, b_takes_turn, _) = b;
            a_takes_turn.time_score.cmp(&b_takes_turn.time_score)
        });

        let mut turn_taking_entities = Vec::new();
        let mut last_was_visible = false;
        let mut is_first = true;
        let mut is_player_turn = false;

        for (entity, _, _name) in data.iter() {
            let is_visible = is_visible.get(*entity).is_some();
            let is_global = global_turn.get(*entity).is_some();
            let is_player = player.get(*entity).is_some();

            if is_global || is_player || is_visible {
                if is_first {
                    turn_taking_entities.push(*entity);
                    is_player_turn = is_player;
                }
                break;
            }

            is_first = false;

            if is_visible && !last_was_visible {
                break;
            }

            turn_taking_entities.push(*entity);
            last_was_visible = is_visible;
        }

        for entity in turn_taking_entities.iter() {
            wants_to_take_turn.insert(*entity, WantsToTakeTurn).expect("Unable to insert intent");
        }

        if is_player_turn {
            RunState::AwaitingInput
        } else {
            RunState::WorldTurn
        }
    }
}