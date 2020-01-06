use std::cmp::{max, min};

use rltk::{Console, RandomNumberGenerator, RGB, Rltk};

use super::{Rect, WINDOW_HEIGHT, WINDOW_WIDTH};

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

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }
}

/// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
/// This gives a handful of random rooms and corridors joining them together.
pub fn new_map_rooms_and_corridors() -> Map {
    let mut tiles = vec![TileType::Wall; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::seeded(1);


    let apply_room_to_map = |room: &Rect| => {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                tiles[xy_idx(x, y)] = TileType::Floor;
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

    for _i in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, WINDOW_WIDTH - w - 1) - 1;
        let y = rng.roll_dice(1, WINDOW_HEIGHT - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let ok = rooms.iter().all(|other_room| !new_room.intersect(other_room));

        if ok {
            apply_room_to_map(&new_room, &mut tiles);

            let prev_room_or_none = rooms.last();

            if let Some(prev_room) = prev_room_or_none {
                let new = new_room.center();
                let prev = prev_room.center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut tiles, prev.x, new.x, prev.y);
                    apply_vertical_tunnel(&mut tiles, prev.y, new.y, new.x);
                } else {
                    apply_horizontal_tunnel(&mut tiles, prev.x, new.x, new.y);
                    apply_vertical_tunnel(&mut tiles, prev.y, new.y, prev.x);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, tiles)
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
