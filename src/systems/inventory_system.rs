extern crate specs;

use specs::prelude::*;

use crate::{GameLog, InBackpack, Name, Position, WantsToPickUp};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToPickUp>,
        WriteStorage<'a, InBackpack>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut game_log,
            mut positions,
            mut wants_to_pick_up,
            mut in_backpack,
            names) = data;


        for pick_up in wants_to_pick_up.join() {
            positions.remove(pick_up.item);
            in_backpack
                .insert(pick_up.item, InBackpack { owner: pick_up.collected_by })
                .expect("Unable to insert backpack entry");

            if pick_up.collected_by == *player_entity {
                let item_name = &names.get(pick_up.item).unwrap().name;
                game_log.entries.insert(0, format!("You picked up {}!", item_name));
            }
        }

        wants_to_pick_up.clear();
    }
}