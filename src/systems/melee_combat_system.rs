extern crate specs;

use specs::prelude::*;

use crate::{CanMelee, CombatStats, GameLog, Name, SuffersDamage, TakesTurn, WantsToMelee};

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

                suffers_damage
                    .insert(wants_melee.target, SuffersDamage { amount: damage })
                    .expect("Unable to do damage");
            }
        }

        wants_melee.clear();
    }
}