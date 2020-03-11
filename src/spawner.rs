use rltk::{Algorithm2D, Point, Rect, RGB};
use specs::{Entity, World};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use crate::{AreaOfEffect, BlocksTile, CanMelee, CanMove, CombatStats, Confusion, Consumable, DEBUG, GlobalTurn, GlobalTurnTimeScore, InBackpack, InflictsDamage, Item, Map, Monster, Name, Player, Position, ProvidesHealing, Ranged, Renderable, RNG, SerializeMe, TakesTurn, Viewshed};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;
const DROP_OFFSET: i32 = 3;

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player)
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
        .with(TakesTurn { time_score: 0 })
        .with(CanMove { time_cost: 20 })
        .with(CanMelee { time_cost: 110 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}


pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    if RNG.flip_coin() {
        orc(ecs, x, y)
    } else {
        goblin(ecs, x, y)
    }
}

static mut COUNTER: i32 = 1;

pub fn orc(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('o'), "Orc")
}

pub fn goblin(ecs: &mut World, x: i32, y: i32) {
    monster(ecs, x, y, rltk::to_cp437('g'), "Goblin")
}

fn monster<S: ToString>(ecs: &mut World, x: i32, y: i32, glyph: u8, name: S) {
    let monster_builder = ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Monster)
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(BlocksTile)
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .with(TakesTurn { time_score: 0 })
        .with(CanMove { time_cost: 30 })
        .with(CanMelee { time_cost: 180 })
        .marked::<SimpleMarker<SerializeMe>>();


    if DEBUG {
        unsafe {
            monster_builder.with(Name { name: format!("{} #{}", name.to_string(), COUNTER) }).build();
            COUNTER += 1
        };
    } else {
        monster_builder.with(Name { name: name.to_string() }).build();
    }
}

pub fn random_item(ecs: &mut World, x: i32, y: i32) {
    match RNG.roll_die(4) {
        1 => { health_potion(ecs, x, y) }
        2 => { fireball_scroll(ecs, x, y) }
        3 => { confusion_scroll(ecs, x, y) }
        _ => { magic_missile_scroll(ecs, x, y) }
    }
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    health_potion_base(ecs)
        .with(Position { x, y })
        .build();
}

pub fn health_potion_in_pack(ecs: &mut World, owner: Entity) {
    health_potion_base(ecs)
        .with(InBackpack { owner })
        .build();
}

pub fn health_potion_base(ecs: &mut World) -> EntityBuilder {
    ecs
        .create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Health Potion".to_string() })
        .with(Item)
        .with(Consumable)
        .with(ProvidesHealing {
            heal_amount: 8
        })
        .marked::<SimpleMarker<SerializeMe>>()
}

pub fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    magic_missile_scroll_base(ecs)
        .with(Position { x, y })
        .build();
}

pub fn magic_missile_scroll_base(ecs: &mut World) -> EntityBuilder {
    ecs
        .create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Magic Missile Scroll".to_string() })
        .with(Item)
        .with(Consumable)
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
}

pub fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    fireball_scroll_base(ecs)
        .with(Position { x, y })
        .build();
}

fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    confusion_scroll_base(ecs)
        .with(Position { x, y })
        .build();
}

fn confusion_scroll_in_pack(ecs: &mut World, owner: Entity) {
    confusion_scroll_base(ecs)
        .with(InBackpack { owner })
        .build();
}

fn confusion_scroll_base(ecs: &mut World) -> EntityBuilder {
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Confusion Scroll".to_string() })
        .with(Item)
        .with(Consumable)
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
}

fn magic_missile_scroll_in_pack(ecs: &mut World, owner: Entity) {
    magic_missile_scroll_base(ecs)
        .with(InBackpack { owner })
        .build();
}

fn fireball_scroll_in_pack(ecs: &mut World, owner: Entity) {
    fireball_scroll_base(ecs)
        .with(InBackpack {
            owner,
        })
        .build();
}

fn fireball_scroll_base(ecs: &mut World) -> EntityBuilder {
    ecs
        .create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name { name: "Fireball Scroll".to_string() })
        .with(Item)
        .with(Consumable)
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
}

pub fn spawn_map(ecs: &mut World, map: &Map) {
    let pt = map.rooms.first().unwrap().center();

    ecs.insert(Point::new(pt.x, pt.y));
    let player = player(ecs, pt.x, pt.y);

    health_potion_in_pack(ecs, player);
    confusion_scroll_in_pack(ecs, player);
    magic_missile_scroll_in_pack(ecs, player);

    for _ in 0..3 {
        fireball_scroll_in_pack(ecs, player);
    }

    ecs.insert(player);

    let rooms = map.rooms.iter().skip(1);
    for room in rooms {
        spawn_room(ecs, map, room);
    }
}

fn spawn_room(ecs: &mut World, map: &Map, room: &Rect) {
    let monster_count = RNG.inclusive_range(0, MAX_MONSTERS + DROP_OFFSET) - DROP_OFFSET;
    let item_count = RNG.inclusive_range(0, MAX_ITEMS + DROP_OFFSET) - DROP_OFFSET;

    let monster_spawn_points = get_spawn_points(map, monster_count, room);
    let item_spawn_points = get_spawn_points(map, item_count, room);

    for idx in monster_spawn_points {
        let pt = map.index_to_point2d(idx);
        random_monster(ecs, pt.x, pt.y);
    }

    for idx in item_spawn_points {
        let pt = map.index_to_point2d(idx);
        random_item(ecs, pt.x, pt.y);
    }
}

fn get_spawn_points(map: &Map, count: i32, room: &Rect) -> Vec<usize> {
    let mut spawn_points: Vec<usize> = Vec::new();
    for _i in 0..count {
        let mut added = false;
        while !added {
            let x = room.x1 + RNG.range(1, i32::abs(room.x2 - room.x1) - 2);
            let y = room.y1 + RNG.range(1, i32::abs(room.y2 - room.y1) - 2);
            let idx = map.xy_idx(x, y);
            if !spawn_points.contains(&idx) {
                spawn_points.insert(0, idx);
                added = true;
            }
        }
    }

    spawn_points
}

pub fn spawn_global_turn(ecs: &mut World) {
    ecs
        .create_entity()
        .with(Name { name: "Global Turn".to_string() })
        .with(GlobalTurn)
        .with(TakesTurn { time_score: 0 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    ecs.insert(GlobalTurnTimeScore { time_score: 0 });
}