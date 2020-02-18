extern crate rltk;
extern crate specs;

use specs::prelude::*;

use crate::{console_log, GlobalTurn, GlobalTurnTimeScore, TakesTurn, WantsToTakeTurn};

pub struct GlobalTurnSystem;

impl<'a> System<'a> for GlobalTurnSystem {
    type SystemData = (
        WriteStorage<'a, TakesTurn>,
        ReadStorage<'a, WantsToTakeTurn>,
        ReadStorage<'a, GlobalTurn>,
        WriteExpect<'a, GlobalTurnTimeScore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut takes_turn,
            wants_to_take_turn,
            global_turn,
            mut global_turn_time_score,
        ) = data;

        const TIME_SCORE_LIMIT: u32 = 1000;

        let turn_time_score = (&takes_turn).join().min_by(|x, y| x.time_score.cmp(&y.time_score)).unwrap().time_score;

        if turn_time_score > TIME_SCORE_LIMIT {
            for mut takes_turn in (&mut takes_turn).join() {
                takes_turn.time_score -= turn_time_score;
            }

            global_turn_time_score.time_score -= turn_time_score;
        }

        for (mut takes_turn, _, _) in (&mut takes_turn, &wants_to_take_turn, &global_turn).join() {
            takes_turn.time_score += 100;

            global_turn_time_score.time_score = takes_turn.time_score;

            console_log(format!("       GlobalTurn time_score ({})", global_turn_time_score.time_score));
        }
    }
}