extern crate specs;

use specs::prelude::*;

use crate::{CombatStats, GameLog, Name, Player, RunStateHolder, SuffersDamage};

pub struct DamageSystem;

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SuffersDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut stats,
            mut damage
        ) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount;
        }

        damage.clear();
    }
}


pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let names = ecs.read_storage::<Name>();
        let entities = ecs.entities();
        let players = ecs.read_storage::<Player>();
        let mut game_log = ecs.write_resource::<GameLog>();
        let run_state_holder = ecs.read_resource::<RunStateHolder>();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp <= 0 {
                let player_or_null = players.get(entity);

                match player_or_null {
                    None => {
                        let victim_name = names.get(entity);
                        if let Some(victim_name) = victim_name {
                            game_log.add(format!("{} is dead", victim_name.name));
                        }
                        dead.push(entity);
                    }
                    Some(_player) => {
                        if run_state_holder.run_state.is_turn() {
                            game_log.add("You are dead".to_string());
                        }
                    }
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}