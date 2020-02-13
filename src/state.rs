use rltk::{GameState, render_draw_buffer, Rltk};
use specs::prelude::*;
use specs::WorldExt;

use crate::{AreaOfEffect, damage_system, DamageSystem, gui, ItemCollectionSystem, ItemDropSystem, ItemMenuResult, ItemUseSystem, MapIndexingSystem, MeleeCombatSystem, MonsterAI, player, Ranged, RangedTargetDrawerSettings, RangedTargetResult, render_camera, RltkExt, VisibilitySystem, WantsToDrop, WantsToUseItem};

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
    pub systems: SysRunner,
}

impl State {
    fn get_run_state(&mut self) -> RunState {
        let run_state = self.ecs.fetch::<RunState>();
        *run_state
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.ext_cls_all();

        render_camera(&self.ecs, context);
        gui::draw_ui(&self.ecs, context);

        let mut new_run_state = self.get_run_state();

        match new_run_state {
            RunState::PreRun => {
                self.systems.run(&mut self.ecs);
                new_run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                new_run_state = player::player_input(self, context);
            }
            RunState::PlayerTurn => {
                self.systems.run(&mut self.ecs);
                new_run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.systems.run(&mut self.ecs);
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

        render_draw_buffer(context);
    }
}

pub struct SysRunner {
    dispatcher: Dispatcher<'static, 'static>,
}

impl SysRunner {
    pub fn new() -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(VisibilitySystem, "vis", &[])
            .with(MonsterAI, "mob", &[])
            .with(MapIndexingSystem, "map_index", &[])
            .with(MeleeCombatSystem, "melee_combat", &[])
            .with(ItemCollectionSystem, "pick_up", &[])
            .with(ItemUseSystem, "use_item", &[])
            .with(ItemDropSystem, "drop", &[])
            .with(DamageSystem, "damage", &["melee_combat", "use_item"])
            .build();


        SysRunner { dispatcher }
    }

    pub fn run(&mut self, ecs: &mut World) {
        self.dispatcher.dispatch(ecs);
        ecs.maintain();
    }
}

