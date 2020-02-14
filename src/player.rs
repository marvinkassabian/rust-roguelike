use std::cmp::{max, min};

use rltk::{console, Point, VirtualKeyCode};
use specs::prelude::*;

use crate::{Context, GameLog, Item, Map, RunState, WantsToMelee, WantsToPickUp};

use super::{CombatStats, Player, Position, State, Viewshed};

pub fn player_input(state: &mut State, context: &mut Context) -> RunState {
    match context.rltk.key {
        None => { return RunState::AwaitingInput; }
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut state.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::L => try_move_player(1, 0, &mut state.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::J => try_move_player(0, -1, &mut state.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::K => try_move_player(0, 1, &mut state.ecs),

            VirtualKeyCode::Y => try_move_player(-1, -1, &mut state.ecs),

            VirtualKeyCode::U => try_move_player(1, -1, &mut state.ecs),

            VirtualKeyCode::N => try_move_player(1, 1, &mut state.ecs),

            VirtualKeyCode::B => try_move_player(-1, 1, &mut state.ecs),

            VirtualKeyCode::G => get_item(&mut state.ecs),
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            VirtualKeyCode::PageUp => {
                try_scroll_game_log(&mut state.ecs, 1);
                return RunState::AwaitingInput;
            }
            VirtualKeyCode::PageDown => {
                try_scroll_game_log(&mut state.ecs, -1);
                return RunState::AwaitingInput;
            }
            _ => return RunState::AwaitingInput,
        },
    }

    RunState::PlayerTurn
}


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
            let is_target = target_or_none.is_some();
            if is_target {
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


fn try_scroll_game_log(ecs: &mut World, delta: i32) {
    let mut game_log = ecs.write_resource::<GameLog>();

    game_log.move_index(delta);
}

fn get_item(ecs: &mut World) {
    let player_position = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();

    let mut picked_up_item_or_none: Option<Entity> = None;
    for (_item, entity, position) in (&items, &entities, &positions).join() {
        if position.x == player_position.x && position.y == player_position.y {
            picked_up_item_or_none = Some(entity);
            break;
        }
    }

    let mut game_log = ecs.fetch_mut::<GameLog>();

    match picked_up_item_or_none {
        None => game_log.add("There is nothing to pick up.".to_string()),
        Some(entity) => {
            let mut wants_to_pick_up = ecs.write_storage::<WantsToPickUp>();
            wants_to_pick_up.insert(entity, WantsToPickUp {
                collected_by: *player_entity,
                item: entity,
            }).expect("Unable to insert WantsToPickUp");
        }
    }
}
