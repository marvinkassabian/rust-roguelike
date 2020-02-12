use rltk::{GameState, Rltk};
use specs::prelude::*;
use specs::WorldExt;

use crate::{AreaOfEffect, CONSOLE_INDEX, damage_system, DamageSystem, gui, ItemCollectionSystem, ItemDropSystem, ItemMenuResult, ItemUseSystem, Map, map, MapIndexingSystem, MeleeCombatSystem, MonsterAI, player, Position, Ranged, RangedTargetDrawerSettings, RangedTargetResult, Renderable, RltkExt, VisibilitySystem, WantsToDrop, WantsToUseItem};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity, radius: Option<i32> },
}

impl RunState {
    pub fn is_turn(&self) -> bool {
        match self {
            RunState::PlayerTurn |
            RunState::MonsterTurn => true,
            _ => false,
        }
    }
}

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls_all();

        draw_entities(&self.ecs, context);
        map::draw_map(&self.ecs, context);
        gui::draw_ui(&self.ecs, context);

        let mut new_run_state;
        {
            let run_state = self.ecs.fetch::<RunState>();
            new_run_state = *run_state;
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_run_state = player::player_input(self, context);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let item_menu_result = gui::show_inventory(self, context);

                match item_menu_result {
                    ItemMenuResult::Cancel => new_run_state = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Selected(selected_item) => {
                        let player_entity = self.ecs.read_resource::<Entity>();
                        match self.ecs.read_storage::<Ranged>().get(selected_item) {
                            Some(ranged) => {
                                let mut radius: Option<i32> = None;
                                if let Some(area_of_effect) = self.ecs.read_storage::<AreaOfEffect>().get(selected_item) {
                                    radius = Some(area_of_effect.radius);
                                }

                                new_run_state = RunState::ShowTargeting {
                                    item: selected_item,
                                    range: ranged.range,
                                    radius,
                                }
                            }
                            None => {
                                let mut want_to_use_items = self.ecs.write_storage::<WantsToUseItem>();
                                want_to_use_items.insert(*player_entity, WantsToUseItem {
                                    item: selected_item,
                                    target: None,
                                }).expect("Unable to insert intent");
                                new_run_state = RunState::PlayerTurn;
                            }
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let drop_item_menu_result = gui::show_drop_item_menu(self, context);

                match drop_item_menu_result {
                    ItemMenuResult::Cancel => new_run_state = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Selected(selected_item) => {
                        let player_entity = self.ecs.read_resource::<Entity>();
                        let mut wants_to_drop = self.ecs.write_storage::<WantsToDrop>();
                        wants_to_drop.insert(*player_entity, WantsToDrop {
                            item: selected_item,
                        }).expect("Unable to insert intent");
                        new_run_state = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting { range, item, radius } => {
                let target_result = gui::ranged_target(
                    self,
                    context,
                    RangedTargetDrawerSettings {
                        range,
                        radius,
                    });

                match target_result {
                    RangedTargetResult::Cancel => new_run_state = RunState::AwaitingInput,
                    RangedTargetResult::NoResponse => {}
                    RangedTargetResult::Selected(target) => {
                        let player_entity = self.ecs.read_resource::<Entity>();
                        let mut want_to_use_items = self.ecs.write_storage::<WantsToUseItem>();
                        want_to_use_items.insert(*player_entity, WantsToUseItem {
                            item,
                            target: Some(target),
                        }).expect("Unable to insert intent");
                        new_run_state = RunState::PlayerTurn;
                    }
                }
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_run_state;
        }

        damage_system::delete_the_dead(&mut self.ecs);
    }
}

impl State {
    pub fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);

        let mut melee_combat = MeleeCombatSystem {};
        melee_combat.run_now(&self.ecs);

        let mut pick_up = ItemCollectionSystem {};
        pick_up.run_now(&self.ecs);

        let mut drink_potion = ItemUseSystem {};
        drink_potion.run_now(&self.ecs);

        let mut drop = ItemDropSystem {};
        drop.run_now(&self.ecs);

        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

fn draw_entities(ecs: &World, context: &mut Rltk) {
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let map = ecs.fetch::<Map>();
    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
    data.sort_by(|a, b| {
        let (_, a_render) = a;
        let (_, b_render) = b;
        b_render.render_order.cmp(&a_render.render_order)
    });

    for (pos, render) in data {
        if map.is_visible(pos.x, pos.y) {
            context.layered_set(pos.x, pos.y, render.fg, render.bg, render.glyph, 2, false);
        }
    }


    context.set_active_console(CONSOLE_INDEX.base);
}
