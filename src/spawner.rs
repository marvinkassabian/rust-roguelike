use rltk::{Algorithm2D, Point, RGB};
use specs::{Entity, World};
use specs::prelude::*;

use crate::{BlocksTile, CombatStats, Item, Map, Monster, Name, Player, Position, Potion, Random, Rect, Renderable, Viewshed};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;
const SPAWN_OFFSET: i32 = 2;

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name { name: "Player".to_string() })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build()
}


pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let coin_flip: bool;
    {
        let mut rng = ecs.write_resource::<Random>();
        coin_flip = rng.flip_coin();
    }
    if coin_flip {
        orc(ecs, x, y)
    } else {
        goblin(ecs, x, y)
    }
}

pub fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc")
}


pub fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin")
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: u8, name: S) {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Monster {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name { name: name.to_string() })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .build();
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Name { name: "Health Potion".to_string() })
        .with(Item {})
        .with(Potion {
            heal_amount: 8
        })
        .build();
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let pt = map.rooms.first().unwrap().center();

    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);
    ecs.insert(player);

    let rooms = map.rooms.iter().skip(1);
    for room in rooms {
        spawn_room(ecs, map, room);
    }
}

fn spawn_room(ecs: &mut World, map: &Map, room: &Rect) {
    let monster_spawn_points: Vec<usize>;
    let item_spawn_points: Vec<usize>;
    {
        let mut rng = ecs.write_resource::<Random>();
        let monster_count = rng.range(0, MAX_MONSTERS) - SPAWN_OFFSET;
        let item_count = rng.range(0, MAX_ITEMS) - SPAWN_OFFSET;

        monster_spawn_points = get_spawn_points(map, &mut rng, monster_count, room);
        item_spawn_points = get_spawn_points(map, &mut rng, item_count, room);
    }

    for idx in monster_spawn_points {
        let pt = map.index_to_point2d(idx as i32);
        random_monster(ecs, pt.x, pt.y);
    }

    for idx in item_spawn_points {
        let pt = map.index_to_point2d(idx as i32);
        health_potion(ecs, pt.x, pt.y);
    }
}

fn get_spawn_points(map: &Map, rng: &mut Random, count: i32, room: &Rect) -> Vec<usize> {
    let mut spawn_points: Vec<usize> = Vec::new();
    for _i in 0..count {
        let mut added = false;
        while !added {
            let x = room.x1 + rng.range(1, i32::abs(room.x2 - room.x1) - 2);
            let y = room.y1 + rng.range(1, i32::abs(room.y2 - room.y1) - 2);
            let idx = map.xy_idx(x, y);
            if !spawn_points.contains(&idx) {
                spawn_points.insert(0, idx);
                added = true;
            }
        }
    }

    spawn_points
}