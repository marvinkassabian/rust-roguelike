extern crate specs;

use specs::prelude::*;

use crate::{GlobalTurnTimeScore, TakesTurn, WantsToWait};

pub struct WaitSystem;

impl<'a> System<'a> for WaitSystem {
    type SystemData = (
        WriteStorage<'a, WantsToWait>,
        WriteStorage<'a, TakesTurn>,
        ReadExpect<'a, GlobalTurnTimeScore>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut wants_to_wait,
            mut takes_turn,
            global_turn_time_score,
        ) = data;

        let turn_time_score = global_turn_time_score.time_score;

        for (_wants_to_wait, mut takes_turn) in (&wants_to_wait, &mut takes_turn).join() {
            takes_turn.time_score = turn_time_score + 1;
        }

        wants_to_wait.clear();
    }
}
