use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use wooting_sdk::rgb::{self, RgbKeyboard};

mod all_keys;
mod color;
mod direction;
mod game;
mod menu;
mod snake;
mod snake_board;
mod sound_manager;
mod utils;

use all_keys::ALL_KEYS;
use color::*;
use sound_manager::SoundManager;

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

fn animate_keys_demo() {
    println!(
        "Keyboard connected? {}",
        rgb::is_wooting_keyboard_connected()
    );
    let mut keyboard = RgbKeyboard::default();
    utils::clear(&mut keyboard, WHITE);
    sleep(Duration::from_millis(500));
    utils::clear(&mut keyboard, RED);
    sleep(Duration::from_millis(500));
    utils::clear(&mut keyboard, BLUE);
    sleep(Duration::from_millis(500));
    let mut rng = rand::thread_rng();
    for _times in 0..10 {
        for i in 0..6 {
            utils::clear(&mut keyboard, WHITE);
            let color: Color;
            if rng.gen::<f32>() > 0.5 {
                color = RED;
            } else {
                color = BLUE;
            }
            utils::column(&mut keyboard, i, color);
            //sleep(Duration::from_millis(10));
        }
    }

    utils::clear(&mut keyboard, WHITE);
    for _i in 0..1000 {
        utils::direct_set_key(
            &mut keyboard,
            ALL_KEYS[rng.gen_range(0, ALL_KEYS.len())],
            GREEN * rng.gen(),
        );
    }

    sleep(Duration::from_millis(400));
    println!("Finished!");
}
