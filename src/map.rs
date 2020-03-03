use std::cmp::{max, min};

use rltk::{Algorithm2D, BaseMap, Point, Rect};
use specs::prelude::*;

use crate::RNG;

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
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn count(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn set(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile;
    }

    pub fn safe_set(&mut self, x: i32, y: i32, tile: TileType) {
        if self.is_valid(x, y) {
            self.set(x, y, tile);
        }
    }

    pub fn get(&self, x: i32, y: i32) -> TileType {
        let idx = self.xy_idx(x, y);
        let tile = self.tiles[idx];

        tile
    }

    pub fn safe_get(&self, x: i32, y: i32) -> Option<TileType> {
        if self.is_valid(x, y) {
            let tile_type = self.get(x, y);

            Some(tile_type)
        } else {
            None
        }
    }

    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        self.is_valid_idx(idx) && x < self.width && y < self.height && x >= 0 && y >= 0
    }

    pub fn is_valid_idx(&self, idx: usize) -> bool {
        idx < self.count()
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);

        self.visible_tiles[idx]
    }

    pub fn is_revealed(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);

        self.revealed_tiles[idx]
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        let idx = self.xy_idx(x, y);
        let is_blocked = self.blocked[idx];

        is_blocked
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if !self.is_valid(x, y) {
            return false;
        }

        let is_blocked = self.is_blocked(x, y);

        !is_blocked
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new(width: i32, height: i32, tile_type: TileType) -> Map {
        let map_count = (width * height) as usize;
        Map {
            tiles: vec![tile_type; map_count],
            rooms: Vec::new(),
            width,
            height,
            revealed_tiles: vec![false; map_count],
            visible_tiles: vec![false; map_count],
            blocked: vec![false; map_count],
            tile_content: vec![Vec::new(); map_count],
        }
    }
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> usize {
        (pt.y * self.width + pt.x) as usize
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        Point {
            x: idx as i32 % self.width,
            y: idx as i32 / self.width,
        }
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut available_exits: Vec::<(usize, f32)> = Vec::new();
        let pt = self.index_to_point2d(idx);

        const CARDINAL_DISTANCE: f32 = 1.0;
        //const DIAGONAL_DISTANCE: f32 = 1.454;
        let deltas = [
            (0, -1, CARDINAL_DISTANCE),
            (0, 1, CARDINAL_DISTANCE),
            (1, 0, CARDINAL_DISTANCE),
            (-1, 0, CARDINAL_DISTANCE),
            /*
            (1, -1, DIAGONAL_DISTANCE),
            (1, 1, DIAGONAL_DISTANCE),
            (-1, -1, DIAGONAL_DISTANCE),
            (-1, 1, DIAGONAL_DISTANCE),
            */
        ];

        for delta in deltas.iter() {
            let
                (delta_x, delta_y, delta_cost) = delta;
            let new_x = pt.x + *delta_x;
            let new_y = pt.y + *delta_y;
            if self.is_exit_valid(new_x, new_y) {
                let idx = self.xy_idx(new_x, new_y);
                available_exits.push((idx, *delta_cost));
            }
        }

        available_exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
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
    const FRAME_WIDTH: i32 = 1;

    for _i in 0..MAX_ROOMS {
        let w = RNG.range(MIN_SIZE, MAX_SIZE);
        let h = RNG.range(MIN_SIZE, MAX_SIZE);
        let x = RNG.range(FRAME_WIDTH, map.width - w - FRAME_WIDTH);
        let y = RNG.range(FRAME_WIDTH, map.height - h - FRAME_WIDTH);
        let new_room = Rect::with_size(x, y, w, h);

        let ok = map.rooms.iter().all(|other_room| !new_room.intersect(other_room));

        if ok {
            apply_room_to_map(&mut map, &new_room);

            let prev_room_or_none = map.rooms.last();

            if let Some(prev_room) = prev_room_or_none {
                let new = new_room.center();
                let prev = prev_room.center();

                if RNG.flip_coin() {
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