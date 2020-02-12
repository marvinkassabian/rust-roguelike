extern crate specs;

use std::cmp::min;

use rltk::Algorithm2D;
use specs::prelude::*;

use crate::{AreaOfEffect, CombatStats, Confusion, Consumable, GameLog, InflictsDamage, Map, Name, ProvidesHealing, SuffersDamage, WantsToUseItem};

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a, WantsToUseItem>,
        WriteStorage<'a, CombatStats>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SuffersDamage>,
        ReadStorage<'a, AreaOfEffect>,
        WriteStorage<'a, Confusion>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut game_log,
            map,
            entities,
            names,
            provides_healing,
            mut wants_to_use_items,
            mut combat_stats,
            consumables,
            inflicts_damage,
            mut suffers_damage,
            aoe,
            confusion,
        ) = data;

        for (user_entity, use_item) in (&entities, &wants_to_use_items).join() {
            let mut used_item = false;
            let item_entity = use_item.item;

            let mut targets: Vec<Entity> = Vec::new();
            match use_item.target {
                None => targets.push(*player_entity),
                Some(target) => {
                    let area_of_effect = aoe.get(use_item.item);
                    match area_of_effect {
                        None => {
                            let target_idx = map.point2d_to_index(target) as usize;
                            let mut hit_entities = map.tile_content[target_idx].to_vec();

                            targets.append(&mut hit_entities);
                        }
                        Some(area_of_effect) => {
                            let blast_tiles = rltk::field_of_view(
                                target,
                                area_of_effect.radius,
                                &*map);

                            let valid_blast_tiles = blast_tiles
                                .iter()
                                .filter(|p| map.in_bounds(**p));

                            let mut hit_entities = valid_blast_tiles
                                .flat_map(|tile| -> &Vec<Entity> {
                                    let target_idx = map.point2d_to_index(*tile) as usize;
                                    return &map.tile_content[target_idx];
                                })
                                .map(|e| *e)
                                .collect::<Vec<Entity>>();

                            targets.append(&mut hit_entities);
                        }
                    }
                }
            }

            let stat_targets = targets.iter().filter(|e| combat_stats.get(**e).is_some()).collect::<Vec<&Entity>>();

            let heal_item = provides_healing.get(item_entity);
            if let Some(heal_item) = heal_item {
                for target in stat_targets.iter() {
                    if let Some(stats) = combat_stats.get_mut(**target) {
                        stats.hp = min(stats.max_hp, stats.hp + heal_item.heal_amount);

                        used_item = true;

                        if user_entity == *player_entity {
                            let item_name = &names.get(item_entity).unwrap().name;
                            game_log.add(format!("You use {}, healing {} hp.", item_name, heal_item.heal_amount));
                        }
                    }
                }
            }

            let damage_item = inflicts_damage.get(item_entity);
            if let Some(damage_item) = damage_item {
                for target in stat_targets.iter() {
                    suffers_damage
                        .insert(
                            **target,
                            SuffersDamage {
                                amount: damage_item.damage
                            })
                        .expect("Unable to insert");

                    used_item = true;

                    if user_entity == *player_entity {
                        let item_name = &names.get(item_entity).unwrap().name;
                        let mob_name = &names.get(**target).unwrap().name;
                        game_log.add(format!("You use {} on {}, inflicting {} hp.", item_name, mob_name, damage_item.damage));
                    }
                }
            }

            let mut mobs_to_confuse = Vec::new();

            let confusion_item = confusion.get(item_entity);
            if let Some(_confusion_item) = confusion_item {
                for target in stat_targets.iter() {
                    mobs_to_confuse.push(target);
                    if user_entity == *player_entity {
                        let item_name = &names.get(item_entity).unwrap().name;
                        let mob_name = &names.get(**target).unwrap().name;
                        game_log.add(format!("You use {} on {}, confusing them.", item_name, mob_name));
                    }
                }
            }

            if used_item {
                if let Some(_consumable) = consumables.get(item_entity) {
                    entities.delete(item_entity).expect("Delete failed");
                }
            }
        }

        wants_to_use_items.clear();
    }
}