extern crate rltk;

use specs::prelude::*;

use crate::{CONSOLE_INDEX, Context, InBackpack, MAP_WIDTH, Name, State, WINDOW_HEIGHT};

use self::rltk::{ColorPair, Point, Rect, RGB, VirtualKeyCode};

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected(Entity) }

pub fn show_inventory(state: &mut State, context: &mut Context) -> ItemMenuResult {
    ItemMenuDrawer {
        state,
        context,
        settings: ItemMenuDrawerSettings {
            title: "Inventory",
        },
    }.show_item_selection_menu()
}

pub fn show_drop_item_menu(state: &mut State, context: &mut Context) -> ItemMenuResult {
    ItemMenuDrawer {
        state,
        context,
        settings: ItemMenuDrawerSettings {
            title: "Drop which item?",
        },
    }.show_item_selection_menu()
}

struct ItemMenuDrawer<'a, 'b> {
    state: &'a mut State,
    context: &'a mut Context<'b>,
    settings: ItemMenuDrawerSettings<'a>,
}

struct ItemMenuDrawerSettings<'a> {
    pub title: &'a str,
}

impl<'a, 'b> ItemMenuDrawer<'a, 'b> {
    pub fn show_item_selection_menu(&mut self) -> ItemMenuResult {
        self.context.set_target(CONSOLE_INDEX.ui);

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
            Rect::with_size(INVENTORY_X, y - 2, INVENTORY_WIDTH, (inventory_count + 3) as i32),
            ColorPair::new(plain_fg, bg));

        self.context.print_color(
            Point::new(
                INVENTORY_X + BORDER_TEXT_OFFSET,
                y - 2),
            &self.settings.title,
            ColorPair::new(
                highlight_fg,
                bg));

        self.context.print_color(
            Point::new(
                INVENTORY_X + BORDER_TEXT_OFFSET,
                y + inventory_count as i32 + 1),
            "ESCAPE to cancel",
            ColorPair::new(
                highlight_fg,
                bg));

        let inventory = (&names, &in_backpacks, &entities)
            .join()
            .filter(|(_, in_backpack, _)| {
                in_backpack.owner == *player_entity
            });

        let mut hotkey = 'a' as u8;
        let mut selectable_items: Vec<Entity> = Vec::new();

        for (name, _in_backpack, entity) in inventory {
            self.context.set(Point::new(INVENTORY_X + 2, y), ColorPair::new(plain_fg, bg), rltk::to_cp437('('));
            self.context.set(Point::new(INVENTORY_X + 3, y), ColorPair::new(highlight_fg, bg), hotkey);
            self.context.set(Point::new(INVENTORY_X + 4, y), ColorPair::new(plain_fg, bg), rltk::to_cp437(')'));

            self.context.print_color(Point::new(INVENTORY_X + 6, y), &name.name, ColorPair::new(plain_fg, bg));

            selectable_items.push(entity);
            y += 1;
            hotkey += 1;
        }
        self.context.set_target(CONSOLE_INDEX.base);

        match self.context.rltk.key {
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