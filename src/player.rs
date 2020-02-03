use std::cmp::{max, min};

use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::{Map, RunState, WantsToMelee};

use super::{CombatStats, Player, Position, State, Viewshed};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();

    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in (&entities, &mut players, &mut positions, &mut viewsheds).join() {
        let new_x = pos.x + delta_x;
        let new_y = pos.y + delta_y;
        let new_idx = map.xy_idx(new_x, new_y);

        if !map.is_valid_idx(new_idx) {
            console::log(format!("({}, {}) is not valid", new_x, new_y));
            return;
        }

        let potential_targets = &map.tile_content[new_idx];

        for potential_target in potential_targets.iter() {
            let potential_target = *potential_target;
            let target_or_none = combat_stats.get(potential_target);
            if let Some(_target) = target_or_none {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: potential_target
                        })
                    .expect("Add target failed");
                return;
            }
        }

        if !map.is_blocked(new_x, new_y) {
            pos.x = min(map.width - 1, max(0, new_x));
            pos.y = min(map.height - 1, max(0, new_y));
            viewshed.dirty = true;
            let mut player_pos = ecs.write_resource::<Point>();
            player_pos.x = pos.x;
            player_pos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput; }
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::J => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::K => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
            _ => { return RunState::AwaitingInput; }
        },
    }

    RunState::PlayerTurn
}
