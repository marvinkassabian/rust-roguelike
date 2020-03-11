extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{GlobalTurnTimeScore, MEDIUM_LIFETIME, ParticleBuilder, Position, TakesTurn, WaitCause, WantsToWait};

pub struct WaitSystem;

impl<'a> System<'a> for WaitSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToWait>,
        WriteStorage<'a, TakesTurn>,
        ReadExpect<'a, GlobalTurnTimeScore>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut wants_to_wait,
            mut takes_turn,
            global_turn_time_score,
            mut particle_builder,
            positions,
        ) = data;

        let target_time_score = global_turn_time_score.time_score + 1;

        for (entity, wants_to_wait, mut takes_turn) in (&entities, &wants_to_wait, &mut takes_turn).join() {
            takes_turn.time_score = target_time_score;

            if let Some(position) = positions.get(entity) {
                match wants_to_wait.cause {
                    WaitCause::Confusion => {
                        particle_builder.request_aura(
                            Point::new(position.x, position.y),
                            MEDIUM_LIFETIME,
                            rltk::RGB::named(rltk::MAGENTA),
                            rltk::to_cp437('?'),
                        );
                    }
                    WaitCause::Choice => {
                        particle_builder.request_aura(
                            Point::new(position.x, position.y),
                            MEDIUM_LIFETIME,
                            rltk::RGB::named(rltk::LIGHT_SKY),
                            rltk::to_cp437('♪'),
                        );
                    }
                    WaitCause::Stun => {}
                }
            }
        }

        wants_to_wait.clear();
    }
}
