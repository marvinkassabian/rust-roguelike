use std::cmp::{max, min};

use rltk::{Algorithm2D, BaseMap, Console, Point, RGB, Rltk};
use specs::prelude::*;

use crate::Random;
use crate::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn set(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile;
    }

    pub fn safe_set(&mut self, x: i32, y: i32, tile: TileType) {
        if self.is_valid_idx(x, y) {
            self.set(x, y, tile);
        }
    }

    pub fn get(&self, x: i32, y: i32) -> TileType {
        let idx = self.xy_idx(x, y);
        let tile = self.tiles[idx];

        tile
    }

    pub fn safe_get(&self, x: i32, y: i32) -> Option<TileType> {
        if self.is_valid_idx(x, y) {
            let tile_type = self.get(x, y);

            Some(tile_type)
        } else {
            None
        }
    }

    pub fn is_valid_idx(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        idx < (self.width * self.height) as usize
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);

        self.visible_tiles[idx]
    }

    pub fn new(width: i32, height: i32, tile_type: TileType) -> Map {
        Map {
            tiles: vec![tile_type; (width * height) as usize],
            rooms: Vec::new(),
            width,
            height,
            revealed_tiles: vec![false; (width * height) as usize],
            visible_tiles: vec![false; (width * height) as usize],
        }
    }
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> i32 {
        pt.y * self.width + pt.x
    }

    fn index_to_point2d(&self, idx: i32) -> Point {
        Point {
            x: idx % self.width,
            y: idx / self.width,
        }
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: i32) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, _idx: i32) -> Vec<(i32, f32)> {
        Vec::new()
    }

    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);
        let _distance = rltk::DistanceAlg::Pythagoras.distance2d(p1, p2);

        _distance
    }
}

/// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
/// This gives a handful of random rooms and corridors joining them together.
pub fn new_map_rooms_and_corridors(width: i32, height: i32) -> Map {
    let mut map = Map::new(width, height, TileType::Wall);

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;
    const FRAME_WIDTH: i32 = 3;

    let mut rng = Random::new();

    for _i in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.inclusive_range(FRAME_WIDTH, map.width - w - FRAME_WIDTH);
        let y = rng.inclusive_range(FRAME_WIDTH, map.height - h - FRAME_WIDTH);
        let new_room = Rect::new(x, y, w, h);

        let ok = map.rooms.iter().all(|other_room| !new_room.intersect(other_room));

        if ok {
            apply_room_to_map(&mut map, &new_room);

            let prev_room_or_none = map.rooms.last();

            if let Some(prev_room) = prev_room_or_none {
                let new = new_room.center();
                let prev = prev_room.center();

                if rng.flip_coin() {
                    apply_horizontal_tunnel(&mut map, prev.x, new.x, prev.y);
                    apply_vertical_tunnel(&mut map, prev.y, new.y, new.x);
                } else {
                    apply_horizontal_tunnel(&mut map, prev.x, new.x, new.y);
                    apply_vertical_tunnel(&mut map, prev.y, new.y, prev.x);
                }
            }

            map.rooms.push(new_room);
        }
    }

    fn apply_room_to_map(map: &mut Map, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                map.set(x, y, TileType::Floor);
            }
        }
    }

    fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            map.safe_set(x, y, TileType::Floor);
        }
    }

    fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            map.safe_set(x, y, TileType::Floor);
        }
    }

    map
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for (idx, tile) in map.tiles.iter().enumerate() {
        let pt = map.index_to_point2d(idx as i32);

        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            let bg = RGB::from_f32(0., 0., 0.);

            let is_visible = map.visible_tiles[idx];

            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(0.5, 1.0, 0.5);
                    glyph = if is_visible { rltk::to_cp437('.') } else { rltk::to_cp437('+') };
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                    glyph = rltk::to_cp437('#');
                }
            }

            if !is_visible {
                fg = fg.to_greyscale();
            }

            ctx.set(pt.x, pt.y, fg, bg, glyph);
        }
    }
}
