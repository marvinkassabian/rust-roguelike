extern crate specs;

use specs::prelude::*;

use crate::{GameLog, InBackpack, Name, Position, WantsToDrop};

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (
        ReadExpect::<'a, Entity>,
        WriteExpect::<'a, GameLog>,
        Entities<'a>,
        WriteStorage::<'a, WantsToDrop>,
        ReadStorage::<'a, Name>,
        WriteStorage::<'a, Position>,
        WriteStorage::<'a, InBackpack>);

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut game_log,
            entities,
            mut wants_to_drop,
            names,
            mut positions,
            mut in_backpacks
        ) = data;

        for (entity, to_drop) in (&entities, &wants_to_drop).join() {
            let drop_position: Position;
            { drop_position = *positions.get(entity).unwrap(); }

            positions.insert(to_drop.item, Position {
                x: drop_position.x,
                y: drop_position.y,
            }).expect("Unable to insert position");

            in_backpacks.remove(to_drop.item).expect("Unable to remove from backpack");

            let name = &names.get(to_drop.item).unwrap().name;
            if entity == *player_entity {
                game_log.add(format!("You dropped the {}", name));
            }
        }

        wants_to_drop.clear();
    }
}