use std::cmp::{max, min};

use rltk::{Console, RandomNumberGenerator, RGB, Rltk};

use crate::{PLAYER_START_X, PLAYER_START_Y, Rect};

use super::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    ((y * WINDOW_WIDTH) + x) as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

    for x in 0..WINDOW_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, WINDOW_HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..WINDOW_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WINDOW_WIDTH - 1, y)] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::seeded(1);

    for _i in 0..WINDOW_WIDTH * WINDOW_HEIGHT {
        let x = rng.roll_dice(1, WINDOW_WIDTH - 1);
        let y = rng.roll_dice(1, WINDOW_HEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(PLAYER_START_X, PLAYER_START_Y) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::seeded(1);

    for i in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, WINDOW_WIDTH - w - 1) - 1;
        let y = rng.roll_dice(1, WINDOW_HEIGHT - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let ok = rooms.iter().all(|other_room| !new_room.intersect(other_room));

        if ok {
            apply_room_to_map(&new_room, &mut map);

            let prev_room = rooms.last();

            match prev_room {
                None => {}
                Some(prev_room) => {
                    let new_center = new_room.center();
                    let prev_center = prev_room.center();
                }
            }

            rooms.push(new_room);
        }
    }

    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (WINDOW_WIDTH * WINDOW_HEIGHT) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < (WINDOW_WIDTH * WINDOW_HEIGHT) as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => ctx.set(
                x,
                y,
                RGB::from_f32(0.5, 0.5, 0.5),
                RGB::from_f32(0., 0., 0.),
                rltk::to_cp437('.'),
            ),
            TileType::Wall => ctx.set(
                x,
                y,
                RGB::from_f32(0.0, 1.0, 0.0),
                RGB::from_f32(0., 0., 0.),
                rltk::to_cp437('#'),
            ),
        }

        x += 1;
        if x >= WINDOW_WIDTH {
            x = 0;
            y += 1;
        }
    }
}
