use crate::color::{BLACK, GREEN, WHITE, RED};
use crate::sound_manager::{SoundManager, SoundType};
use crate::utils;
use std::time::Instant;
use wooting_sdk::rgb::RgbKeyboard;

static KEYBOARD_QUERY_TIME_MS: u128 = 20;

static PLAY_KEYS: [wooting_sdk::Key; 4] = [
    wooting_sdk::Key::R,
    wooting_sdk::Key::T,
    wooting_sdk::Key::Y,
    wooting_sdk::Key::U,
];
static EXIT_KEYS: [wooting_sdk::Key; 4] = [
    wooting_sdk::Key::V,
    wooting_sdk::Key::B,
    wooting_sdk::Key::N,
    wooting_sdk::Key::M,
];

pub enum MenuResult {
    Play,
    Exit,
}

fn draw_menu(keyboard: &mut RgbKeyboard, menu_result: &MenuResult) {
    let (play_color, exit_color) = match menu_result {
        MenuResult::Play => (GREEN, WHITE),
        MenuResult::Exit => (WHITE, RED),
    };

    for key in PLAY_KEYS.iter() {
        utils::direct_set_key(keyboard, *key, play_color);
    }
    for key in EXIT_KEYS.iter() {
        utils::direct_set_key(keyboard, *key, exit_color);
    }
}

fn swap_state(menu_result: &mut MenuResult) {
    let swapped_state = match menu_result {
        MenuResult::Play => MenuResult::Exit,
        MenuResult::Exit => MenuResult::Play,
    };
    *menu_result = swapped_state;
}

pub fn run_menu(keyboard: &mut RgbKeyboard, sound_manager: &mut SoundManager) -> MenuResult {
    utils::clear(keyboard, BLACK);

    let mut menu_state = MenuResult::Play;
    draw_menu(keyboard, &menu_state);

    use device_query::{DeviceQuery, DeviceState, Keycode};
    let device_state = DeviceState::new();
    let mut last_time = Instant::now();
    let mut now;
    let mut prev_keys: Vec<Keycode> = Vec::<Keycode>::new();
    loop {
        now = Instant::now();
        let since_last_update = now.duration_since(last_time).as_millis();
        if since_last_update < KEYBOARD_QUERY_TIME_MS {
            std::thread::yield_now();
            continue;
        }
        last_time = now;
        let current_keys = device_state.get_keys();
        for key in current_keys
            .iter()
            // Only do actions on keys, only pressed this frame
            .filter(|cur_key| !prev_keys.iter().any(|prev_key| **cur_key == *prev_key))
        {
            match key {
                Keycode::Escape => {
                    return MenuResult::Exit;
                }
                Keycode::Up | Keycode::Down => {
                    swap_state(&mut menu_state);
                    draw_menu(keyboard, &mut menu_state);
                    sound_manager.play(SoundType::Click);
                }
                Keycode::Enter => {
                    sound_manager.play(SoundType::Click);
                    return menu_state;
                }
                _ => {}
            }
        }
        prev_keys = current_keys;
    }
}
