extern crate rltk;

use specs::prelude::*;

use crate::{CONSOLE_INDEX, InBackpack, MAP_WIDTH, Name, State, WINDOW_HEIGHT};

use self::rltk::{Console, RGB, Rltk, VirtualKeyCode};

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected(Entity) }

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> ItemMenuResult {
    ItemMenuDrawer {
        state: gs,
        context: ctx,
        settings: ItemMenuDrawerSettings {
            title: "Inventory",
        },
    }.show_item_selection_menu()
}

pub fn show_drop_item_menu(gs: &mut State, ctx: &mut Rltk) -> ItemMenuResult {
    ItemMenuDrawer {
        state: gs,
        context: ctx,
        settings: ItemMenuDrawerSettings {
            title: "Drop which item?",
        },
    }.show_item_selection_menu()
}

struct ItemMenuDrawer<'a> {
    state: &'a mut State,
    context: &'a mut Rltk,
    settings: ItemMenuDrawerSettings<'a>,
}

struct ItemMenuDrawerSettings<'a> {
    pub title: &'a str,
}

impl<'a> ItemMenuDrawer<'a> {
    pub fn show_item_selection_menu(&mut self) -> ItemMenuResult {
        self.context.set_active_console(CONSOLE_INDEX.ui);

        let player_entity = self.state.ecs.fetch::<Entity>();
        let names = self.state.ecs.read_storage::<Name>();
        let in_backpacks = self.state.ecs.read_storage::<InBackpack>();
        let entities = self.state.ecs.entities();

        let inventory_count = in_backpacks
            .join()
            .filter(|in_backpack| {
                in_backpack.owner == *player_entity
            })
            .count();

        let mut y = WINDOW_HEIGHT / 2 - (inventory_count / 2) as i32;
        let bg = RGB::named(rltk::BLACK);
        let highlight_fg = RGB::named(rltk::YELLOW);
        let plain_fg = RGB::named(rltk::WHITE);

        const INVENTORY_WIDTH: i32 = 31;
        const INVENTORY_X: i32 = MAP_WIDTH as i32 / 2 - (INVENTORY_WIDTH / 2);
        const BORDER_TEXT_OFFSET: i32 = 3;

        self.context.draw_box(
            INVENTORY_X,
            y - 2,
            INVENTORY_WIDTH,
            (inventory_count + 3) as i32,
            plain_fg,
            bg);

        self.context.print_color(
            INVENTORY_X + BORDER_TEXT_OFFSET,
            y - 2,
            highlight_fg,
            bg,
            &self.settings.title);

        self.context.print_color(
            INVENTORY_X + BORDER_TEXT_OFFSET,
            y + inventory_count as i32 + 1,
            highlight_fg,
            bg,
            "ESCAPE to cancel");

        let inventory = (&names, &in_backpacks, &entities)
            .join()
            .filter(|(_, in_backpack, _)| {
                in_backpack.owner == *player_entity
            });

        let mut j = 'a' as u8;
        let mut selectable_items: Vec<Entity> = Vec::new();

        for (name, _in_backpack, entity) in inventory {
            self.context.set(INVENTORY_X + 2, y, plain_fg, bg, rltk::to_cp437('('));
            self.context.set(INVENTORY_X + 3, y, highlight_fg, bg, j);
            self.context.set(INVENTORY_X + 4, y, plain_fg, bg, rltk::to_cp437(')'));

            self.context.print_color(INVENTORY_X + 6, y, plain_fg, bg, &name.name);

            selectable_items.push(entity);
            y += 1;
            j += 1;
        }

        self.context.set_active_console(CONSOLE_INDEX.base);

        match self.context.key {
            None => ItemMenuResult::NoResponse,
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => ItemMenuResult::Cancel,
                    _ => {
                        let selection = rltk::letter_to_option(key) as usize;

                        let selected_item_or_none = selectable_items.get(selection);

                        match selected_item_or_none {
                            None => ItemMenuResult::NoResponse,
                            Some(selected_item) => ItemMenuResult::Selected(*selected_item),
                        }
                    }
                }
            }
        }
    }
}