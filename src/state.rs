use rltk::{GameState, render_draw_buffer, Rltk};
use specs::prelude::*;
use specs::WorldExt;

use crate::{AreaOfEffect, console_log, Context, cull_dead_particles, DamageSystem, decide_turn, delete_the_dead, GlobalTurnSystem, gui, ItemCollectionSystem, ItemDropSystem, ItemMenuResult, ItemUseSystem, MapIndexingSystem, MeleeCombatSystem, MonsterAI, MovementSystem, ParticleSpawnSystem, player_input, Ranged, RangedTargetDrawerSettings, RangedTargetResult, render_camera, VisibilitySystem, WaitSystem, WantsToDrop, WantsToUseItem};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    DecideTurn,
    WorldTurn,
    PlayerTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity, radius: Option<i32> },
}

impl RunState {
    pub fn is_turn(&self) -> bool {
        match self {
            RunState::PlayerTurn |
            RunState::WorldTurn => true,
            _ => false,
        }
    }
}

pub struct State {
    pub ecs: World,
    pub systems: SysRunner,
}

pub struct RunStateHolder {
    pub run_state: RunState
}

pub struct GlobalTurnTimeScore {
    pub time_score: u32,
}

impl State {
    fn get_run_state(&mut self) -> RunState {
        let run_state_holder = self.ecs.fetch::<RunStateHolder>();
        run_state_holder.run_state
    }

    fn set_run_state(&mut self, new_run_state: RunState) {
        let mut run_state_holder = self.ecs.write_resource::<RunStateHolder>();
        let old_run_state = run_state_holder.run_state;

        if old_run_state != new_run_state {
            console_log(format!("Turn state: {:?} -> {:?}", old_run_state, new_run_state));
        }

        run_state_holder.run_state = new_run_state;
    }
}

impl GameState for State {
    fn tick(&mut self, rltk: &mut Rltk) {
        let context = &mut Context::new(rltk);
        context.cls_all();
        cull_dead_particles(&mut self.ecs, context);


        render_camera(&self.ecs, context);
        gui::draw_ui(&self.ecs, context);

        let mut new_run_state = self.get_run_state();

        match new_run_state {
            RunState::PreRun => {
                self.systems.run(&mut self.ecs);
                new_run_state = RunState::DecideTurn;
            }
            RunState::AwaitingInput => {
                new_run_state = player_input(self, context);
            }
            RunState::DecideTurn => {
                new_run_state = decide_turn(&mut self.ecs);
            }
            RunState::PlayerTurn => {
                self.systems.run(&mut self.ecs);
                new_run_state = RunState::DecideTurn;
            }
            RunState::WorldTurn => {
                self.systems.run(&mut self.ecs);
                new_run_state = RunState::DecideTurn;
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

        self.set_run_state(new_run_state);

        delete_the_dead(&mut self.ecs);

        render_draw_buffer(&mut context.rltk);
    }
}

pub struct SysRunner {
    dispatcher: Dispatcher<'static, 'static>,
}

impl SysRunner {
    pub fn new() -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(MapIndexingSystem, MapIndexingSystem::NAME, &[])
            .with(VisibilitySystem, "vis", &[MapIndexingSystem::NAME])
            .with(GlobalTurnSystem, "global", &[MapIndexingSystem::NAME])
            .with(MonsterAI, MonsterAI::NAME, &[MapIndexingSystem::NAME])
            .with(MovementSystem, MovementSystem::NAME, &[MonsterAI::NAME])
            .with(MeleeCombatSystem, "melee_combat", &[MonsterAI::NAME])
            .with(WaitSystem, "wait", &[MonsterAI::NAME])
            .with(ItemCollectionSystem, "pick_up", &[MonsterAI::NAME])
            .with(ItemUseSystem, "use_item", &[MonsterAI::NAME])
            .with(ItemDropSystem, "drop", &[MonsterAI::NAME])
            .with(DamageSystem, "damage", &["melee_combat", "use_item"])
            .with(ParticleSpawnSystem, "particle", &["damage"])
            .build();

        SysRunner { dispatcher }
    }

    pub fn run(&mut self, ecs: &mut World) {
        self.dispatcher.dispatch(ecs);
        ecs.maintain();
    }
}

