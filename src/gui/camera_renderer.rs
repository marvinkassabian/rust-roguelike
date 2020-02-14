use rltk::{Algorithm2D, ColorPair, Console, Point, RGB};
use specs::prelude::*;

use crate::{Context, Map, Position, Renderable, TileType};

const SHOW_BOUNDARIES: bool = true;
const WALL_HEIGHT: usize = 2;
const ENTITY_HEIGHT: usize = 2;

pub struct CameraRenderer<'camera> {
    pub ecs: &'camera World,
    pub context: &'camera mut Context<'camera>,
}

pub fn render_camera<'a>(ecs: &'a World, context: &'a mut Context<'a>) {
    CameraRenderer {
        ecs,
        context,
    }.render_camera()
}

pub fn get_screen_bounds<'w: 'r, 'r>(ecs: &'w World, context: &'w mut Context<'r>) -> (i32, i32, i32, i32) {
    CameraRenderer {
        ecs,
        context,
    }.get_screen_bounds()
}

impl<'a> CameraRenderer<'a> {
    pub fn render_camera(&mut self) {
        let (min_x, max_x, min_y, max_y) = self.get_screen_bounds();

        self.draw_map(min_x, min_y, max_x, max_y);
        self.draw_entities(min_x, min_y);
    }

    fn draw_map(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        let map = self.ecs.fetch::<Map>();


        let mut y = 0;
        for ty in min_y..max_y {
            let mut x = 0;
            for tx in min_x..max_x {
                if tx >= 0 && tx < map.width && ty >= 0 && ty < map.height {
                    let idx = map.xy_idx(tx, ty);
                    if map.is_revealed(tx, ty) {
                        let (glyph, fg, bg, is_layered) = get_tile_glyph(idx, &*map);

                        if is_layered {
                            self.context.ext_layered_set(Point::new(x, y), ColorPair::new(fg, bg), glyph, WALL_HEIGHT, true);
                        } else {
                            self.context.ext_set(Point::new(x, y), ColorPair::new(fg, bg), glyph);
                        }
                    }
                } else if SHOW_BOUNDARIES {
                    self.context.ext_set(
                        Point::new(x, y),
                        ColorPair::new(RGB::named(rltk::GREY), RGB::named(rltk::BLACK)),
                        rltk::to_cp437('·'));
                }
                x += 1;
            }
            y += 1;
        }
    }

    fn draw_entities(&mut self, min_x: i32, min_y: i32) {
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        let map_width = map.width - 1;
        let map_height = map.height - 1;

        let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
        data.sort_by(|a, b| {
            let (_, a_render) = a;
            let (_, b_render) = b;
            b_render.render_order.cmp(&a_render.render_order)
        });

        for (pos, render) in data.iter() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                let entity_screen_x = pos.x - min_x;
                let entity_screen_y = pos.y - min_y;
                if entity_screen_x > 0 && entity_screen_x < map_width && entity_screen_y > 0 && entity_screen_y < map_height {
                    self.context.ext_layered_set(Point::new(entity_screen_x, entity_screen_y), ColorPair::new(render.fg, render.bg), render.glyph, ENTITY_HEIGHT, true);
                }
            }
        }
    }

    pub fn get_screen_bounds(&mut self) -> (i32, i32, i32, i32) {
        let player_pos = self.ecs.fetch::<Point>();
        let (x_chars, y_chars) = self.context.rltk.get_char_size();

        let center_x = (x_chars / 2) as i32;
        let center_y = (y_chars / 2) as i32;

        let min_x = player_pos.x - center_x;
        let max_x = min_x + x_chars as i32;
        let min_y = player_pos.y - center_y;
        let max_y = min_y + y_chars as i32;

        (min_x, max_x, min_y, max_y)
    }
}

fn get_wall_glyph(map: &Map, x: i32, y: i32) -> char {
    if !map.is_valid(x, y) {
        return '#';
    }

    let deltas = [
        (x, y - 1, 1),
        (x, y + 1, 2),
        (x - 1, y, 4),
        (x + 1, y, 8),
    ];

    let mut mask: u8 = 0;

    for (delta_x, delta_y, flag) in deltas.iter() {
        if map.is_valid(*delta_x, *delta_y) && is_revealed_wall(map, *delta_x, *delta_y) {
            mask += *flag;
        }
    }

    match mask {
        0 => { '•' } // Pillar because we can't see neighbors
        1 => { '║' } // Wall only to the north
        2 => { '║' } // Wall only to the south
        3 => { '║' } // Wall to the north and south
        4 => { '═' } // Wall only to the west
        5 => { '╝' } // Wall to the north and west
        6 => { '╗' } // Wall to the south and west
        7 => { '╣' } // Wall to the north, south and west
        8 => { '═' } // Wall only to the east
        9 => { '╚' } // Wall to the north and east
        10 => { '╔' } // Wall to the south and east
        11 => { '╠' } // Wall to the north, south and east
        12 => { '═' } // Wall to the east and west
        13 => { '╩' } // Wall to the east, west, and south
        14 => { '╦' } // Wall to the east, west, and north
        15 => { '╬' } //Wall to the north, south, east, and west
        _ => { '#' } // We missed one?
    }
}

fn is_revealed_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);

    if map.tiles[idx] != TileType::Wall {
        return false;
    }

    if !map.revealed_tiles[idx] {
        return false;
    }

    return true;
}

fn get_tile_glyph(idx: usize, map: &Map) -> (u8, RGB, RGB, bool) {
    let pt = map.index_to_point2d(idx);
    let tile = map.tiles[idx];

    let mut glyph = rltk::to_cp437(' ');
    let mut fg = RGB::from_f32(0., 0., 0.);
    let mut bg = RGB::named(rltk::GREY26);

    if map.revealed_tiles[idx] {
        match tile {
            TileType::Floor => {
                fg = RGB::from_f32(0.5, 1.0, 0.5);
                glyph = rltk::to_cp437('·')
            }
            TileType::Wall => {
                fg = RGB::from_f32(0.0, 1.0, 0.0);
                glyph = rltk::to_cp437(get_wall_glyph(&map, pt.x, pt.y));
            }
        }
    }

    if !map.visible_tiles[idx] {
        fg = fg.to_greyscale();
    } else {
        bg = RGB::named(rltk::BLACK);
    }

    if tile == TileType::Wall {
        (glyph, fg, bg, true)
    } else {
        (glyph, fg, bg, false)
    }
}