extern crate rltk;
extern crate specs;

use specs::prelude::*;

use crate::{Confusion, console_log, GlobalTurn, GlobalTurnTimeScore, TakesTurn, WantsToTakeTurn};

pub struct GlobalTurnSystem;

impl<'a> System<'a> for GlobalTurnSystem {
    type SystemData = (
        WriteStorage<'a, TakesTurn>,
        ReadStorage<'a, WantsToTakeTurn>,
        ReadStorage<'a, GlobalTurn>,
        WriteExpect<'a, GlobalTurnTimeScore>,
        WriteStorage<'a, Confusion>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut takes_turn,
            wants_to_take_turn,
            global_turn,
            mut global_turn_time_score,
            mut confusions,
            entities,
        ) = data;

        const TIME_SCORE_LIMIT: u32 = 1000;

        let turn_time_score = (&takes_turn).join().min_by(|x, y| x.time_score.cmp(&y.time_score)).unwrap().time_score;

        if turn_time_score > TIME_SCORE_LIMIT {
            for mut takes_turn in (&mut takes_turn).join() {
                takes_turn.time_score -= turn_time_score;
            }

            global_turn_time_score.time_score -= turn_time_score;
        }

        let mut turn_taken = false;
        {
            for (mut global_takes_turn, _, _) in (&mut takes_turn, &wants_to_take_turn, &global_turn).join() {
                turn_taken = true;
                global_takes_turn.time_score += 100;

                global_turn_time_score.time_score = global_takes_turn.time_score;

                console_log(format!("       GlobalTurn time_score ({})", global_turn_time_score.time_score));
            }
        }

        if turn_taken {
            let mut confusions_to_remove = Vec::new();
            {
                for (_, mut confusion, entity) in (&takes_turn, &mut confusions, &entities).join() {
                    confusion.turns -= 1;

                    if confusion.turns <= 0 {
                        confusions_to_remove.push(entity);
                    }
                }
            }
            for entity in confusions_to_remove.iter() {
                confusions.remove(*entity);
            }
        }
    }
}