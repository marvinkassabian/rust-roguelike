use rltk::{ColorPair, RGB, VirtualKeyCode};

use crate::{Context, does_save_exist, RunState, State, TITLE};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MainMenuResult { NoSelection { selected: MainMenuSelection }, Selected { selected: MainMenuSelection } }

pub fn main_menu(state: &mut State, context: &mut Context) -> MainMenuResult {
    let run_state = state.get_run_state();
    let save_exists = does_save_exist();

    context.print_color_centered(15, ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK)), TITLE);

    if let RunState::MainMenu { menu_selection: selection } = run_state {
        if selection == MainMenuSelection::NewGame {
            context.print_color_centered(24, ColorPair::new(RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK)), "Begin New Game");
        } else {
            context.print_color_centered(24, ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)), "Begin New Game");
        }

        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                context.print_color_centered(25, ColorPair::new(RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK)), "Load Game");
            } else {
                context.print_color_centered(25, ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)), "Load Game");
            }
        }

        if selection == MainMenuSelection::Quit {
            context.print_color_centered(26, ColorPair::new(RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK)), "Quit");
        } else {
            context.print_color_centered(26, ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)), "Quit");
        }

        return match context.rltk.key {
            None => MainMenuResult::NoSelection { selected: selection },
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => { MainMenuResult::NoSelection { selected: MainMenuSelection::Quit } }
                    VirtualKeyCode::Up => {
                        let new_selection;
                        match selection {
                            MainMenuSelection::NewGame => new_selection = MainMenuSelection::Quit,
                            MainMenuSelection::LoadGame => new_selection = MainMenuSelection::NewGame,
                            MainMenuSelection::Quit => new_selection = match save_exists {
                                true => MainMenuSelection::LoadGame,
                                false => MainMenuSelection::NewGame
                            }
                        }

                        MainMenuResult::NoSelection { selected: new_selection }
                    }
                    VirtualKeyCode::Down => {
                        let new_selection;
                        match selection {
                            MainMenuSelection::NewGame => new_selection = match save_exists {
                                true => MainMenuSelection::LoadGame,
                                false => MainMenuSelection::Quit
                            },
                            MainMenuSelection::LoadGame => new_selection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => new_selection = MainMenuSelection::NewGame
                        }
                        MainMenuResult::NoSelection { selected: new_selection }
                    }
                    VirtualKeyCode::Return => MainMenuResult::Selected { selected: selection },
                    _ => MainMenuResult::NoSelection { selected: selection }
                }
            }
        };
    }

    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}