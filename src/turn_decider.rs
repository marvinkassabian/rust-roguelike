extern crate rltk;
extern crate specs;

use specs::prelude::*;

use crate::{console_log, GlobalTurn, IsVisible, Name, Player, RunState, TakesTurn, WantsToTakeTurn};

pub struct TurnDecider<'a> {
    pub ecs: &'a World,
}

pub fn decide_turn(ecs: &mut World) -> RunState {
    TurnDecider { ecs }.decide_turn()
}

impl<'a> TurnDecider<'a> {
    pub fn decide_turn(&mut self) -> RunState {
        let (turn_taking_entities, is_player_turn) = self.get_turn_taking_entities();
        self.print_turn_batch(&turn_taking_entities);
        self.post_wants_to_take_turns(&turn_taking_entities);

        if is_player_turn {
            RunState::AwaitingInput
        } else {
            RunState::WorldTurn
        }
    }

    fn get_turn_taking_entities(&mut self) -> (Vec<Entity>, bool) {
        let mut turn_taking_entities = Vec::new();

        let (
            mut takes_turn,
            global_turn,
            player,
            entities,
            names,
            is_visible,
        ) = (
            self.ecs.write_storage::<TakesTurn>(),
            self.ecs.read_storage::<GlobalTurn>(),
            self.ecs.read_storage::<Player>(),
            self.ecs.entities(),
            self.ecs.read_storage::<Name>(),
            self.ecs.read_storage::<IsVisible>(),
        );

        let mut data = (&entities, &mut takes_turn, &names).join().collect::<Vec<_>>();
        data.sort_by(|a, b| {
            let (_, a_takes_turn, _) = a;
            let (_, b_takes_turn, _) = b;
            a_takes_turn.time_score.cmp(&b_takes_turn.time_score)
        });

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

        (turn_taking_entities, is_player_turn)
    }

    fn print_turn_batch(&mut self, turn_taking_entities: &Vec<Entity>) {
        let (
            takes_turn,
            names,
            is_visible,
        ) = (
            self.ecs.read_storage::<TakesTurn>(),
            self.ecs.read_storage::<Name>(),
            self.ecs.read_storage::<IsVisible>(),
        );

        let count = turn_taking_entities.len();

        if count > 1 {
            console_log("Is a batched turn");
            for entity in turn_taking_entities.iter() {
                let takes_turn = takes_turn.get(*entity).unwrap();
                let name = names.get(*entity).unwrap();
                let is_visible = is_visible.get(*entity).is_some();
                console_log(format!("   ({}) {} (is_visible: {})", takes_turn.time_score, name.name, is_visible));
            }
        } else if count == 1 {
            let entity = turn_taking_entities.first().unwrap();
            let takes_turn = takes_turn.get(*entity).unwrap();
            let name = names.get(*entity).unwrap();
            let is_visible = is_visible.get(*entity).is_some();
            console_log(format!("Is ({}) {}'s turn (is_visible: {})", takes_turn.time_score, name.name, is_visible));
        } else {
            console_log("Is a non turn");
        }
    }


    fn post_wants_to_take_turns(&mut self, turn_taking_entities: &Vec<Entity>) {
        let mut wants_to_take_turn = self.ecs.write_storage::<WantsToTakeTurn>();
        wants_to_take_turn.clear();

        for entity in turn_taking_entities.iter() {
            wants_to_take_turn.insert(*entity, WantsToTakeTurn).expect("Unable to insert intent");
        }
    }
}