extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{CanMelee, CombatStats, GameLog, Name, ParticleBuilder, Position, SuffersDamage, TakesTurn, WantsToMelee};

pub struct MeleeCombatSystem;

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SuffersDamage>,
        WriteStorage<'a, TakesTurn>,
        ReadStorage<'a, CanMelee>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut game_log,
            mut wants_melee,
            names,
            combat_stats,
            mut suffers_damage,
            mut takes_turn,
            can_melee,
            positions,
            mut particle_builder,
        ) = data;

        for (wants_melee, name, stats, mut takes_turn, can_melee) in (&wants_melee, &names, &combat_stats, &mut takes_turn, &can_melee).join() {
            takes_turn.time_score += can_melee.time_cost;

            if stats.hp <= 0 {
                continue;
            }

            let target_stats = combat_stats.get(wants_melee.target).unwrap();

            if target_stats.hp <= 0 {
                continue;
            }

            let target_name = names.get(wants_melee.target).unwrap();

            let damage = i32::max(0, stats.power - target_stats.defense);

            if damage == 0 {
                game_log.add(format!(
                    "{} is unable to hurt {}.",
                    &name.name,
                    &target_name.name));
            } else {
                game_log.add(format!(
                    "{} hits {} for {} hp.",
                    &name.name,
                    &target_name.name,
                    damage));

                if let Some(target_position) = positions.get(wants_melee.target) {
                    particle_builder.request_aura(
                        Point::new(target_position.x, target_position.y),
                        300.,
                        rltk::RGB::named(rltk::ORANGE),
                        rltk::to_cp437('‼'),
                    );
                }

                suffers_damage
                    .insert(wants_melee.target, SuffersDamage { amount: damage })
                    .expect("Unable to do damage");
            }
        }

        wants_melee.clear();
    }
}