use rltk::{Algorithm2D, field_of_view, Point};
use specs::prelude::*;

use crate::{IsVisible, Map, Player, Position, Viewshed};

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, IsVisible>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            entities,
            mut viewsheds,
            positions,
            player,
            mut is_visible) = data;

        for (entity, viewshed, position) in (&entities, &mut viewsheds, &positions).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| map.in_bounds(*p));

                let player_entity_or_none = player.get(entity);
                if let Some(_) = player_entity_or_none {
                    for is_visible in map.visible_tiles.iter_mut() {
                        *is_visible = false;
                    };

                    is_visible.clear();

                    for visible_tile in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                        for tile_entity in map.tile_content[idx].iter() {
                            is_visible.insert(*tile_entity, IsVisible).expect("Unable to insert");
                        }
                    }
                }
            }
        }
    }
}