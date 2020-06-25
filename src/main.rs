mod all_keys;
mod color;
mod direction;
mod game;
mod menu;
mod snake;
mod snake_board;
mod sound_manager;
mod utils;

use color::*;
use sound_manager::SoundManager;
use wooting_sdk::rgb::RgbKeyboard;

fn main() {
    let mut keyboard = RgbKeyboard::default();
    let mut sound_manager = SoundManager::new();
    utils::clear(&mut keyboard, BLACK);
    loop {
        let menu_result = menu::run_menu(&mut keyboard, &mut sound_manager);
        match menu_result {
            menu::MenuResult::Play => {
                game::run_game(&mut keyboard, &mut sound_manager);
            }
            menu::MenuResult::Exit => {
                break;
            }
        }
    }
}