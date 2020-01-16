use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::Map;

use super::{Player, Position, State, TileType, Viewshed};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let new_x = pos.x + delta_x;
        let new_y = pos.y + delta_y;
        let tile_type_or_none = map.safe_get(new_x, new_y);
        match tile_type_or_none {
            None => { println!("{}, {}", new_x, new_y) }
            Some(tile_type) => match tile_type {
                TileType::Wall => {}
                _ => {
                    pos.x = min(map.width - 1, max(0, new_x));
                    pos.y = min(map.height - 1, max(0, new_y));
                    viewshed.dirty = true;
                }
            }
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
