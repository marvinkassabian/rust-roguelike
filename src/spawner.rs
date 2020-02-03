use rltk::{console, RGB};
use specs::{Entity, World};
use specs::prelude::*;

use crate::{BlocksTile, CombatStats, Map, Monster, Name, Player, Position, Random, Rect, Renderable, Viewshed};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

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

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let map = ecs.fetch::<Map>();
    let mut monster_spawn_point: Vec<usize> = Vec::new();
    {
        let mut rng = ecs.write_resource::<Random>();
        let monster_count = rng.range(-2, 4);

        console::log(format!("{}", monster_count));
        for _i in 0..monster_count {
            let mut added = false;
            while !added {
                let x = 4;
                added = true;
            }
        }
    }
}