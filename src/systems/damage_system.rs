extern crate specs;

use rltk::Point;
use specs::prelude::*;

use crate::{CombatStats, GameLog, MEDIUM_LIFETIME, Name, ParticleBuilder, Player, Position, RunStateHolder, SuffersDamage};

pub struct DamageSystem;

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SuffersDamage>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut combat_stats,
            mut suffers_damage,
            mut particle_builder,
            positions,
        ) = data;

        for (entity, mut combat_stat, suffer_damage) in (&entities, &mut combat_stats, &suffers_damage).join() {
            if suffer_damage.amount == 0 {
                continue;
            }

            combat_stat.hp -= suffer_damage.amount;

            if let Some(position) = positions.get(entity) {
                particle_builder.request_aura(
                    Point::new(position.x, position.y),
                    MEDIUM_LIFETIME,
                    rltk::RGB::named(rltk::ORANGE),
                    rltk::to_cp437('‼'),
                );
            }
        }

        suffers_damage.clear();
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