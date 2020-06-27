use device_query::{DeviceQuery, DeviceState};
use rand::prelude::*;
use std::time::Instant;

use wooting_sdk::rgb::RgbKeyboard;

use crate::color;
use crate::constants::{TIME_GAME_TICK_MS, TIME_INPUT_POLLING_MS};
use crate::direction;
use crate::snake;
use crate::snake_board;
use crate::sound_manager;
use crate::utils;

use color::*;
use direction::Direction;
use snake::Snake;
use snake_board::*;
use sound_manager::{SoundManager, SoundType};

pub fn run_game(keyboard: &mut RgbKeyboard, sound_manager: &mut SoundManager) {
    utils::clear(keyboard, BLACK);
    let mut snake = Snake::new();
    let mut fruit_pos = (7, 2);
    let mut now;
    let mut last_input_poll_time = Instant::now();
    let mut last_game_tick_time = Instant::now();
    let mut current_input_dir: Option<Direction> = None;
    loop {
        now = Instant::now();
        let time_since_input_poll = now.duration_since(last_input_poll_time).as_millis();
        if time_since_input_poll > TIME_INPUT_POLLING_MS {
            last_input_poll_time = now.clone();
            if let Some(input_dir) = poll_input(&current_input_dir) {
                current_input_dir = Some(input_dir);
            }
        }

        let time_since_game_tick = now.duration_since(last_game_tick_time).as_millis();
        if time_since_game_tick > TIME_GAME_TICK_MS {
            last_game_tick_time = now;
            let should_quit = tick(
                &mut snake,
                &mut fruit_pos,
                keyboard,
                sound_manager,
                &mut current_input_dir,
            );
            if should_quit {
                return;
            }
        }
    }
}

pub fn poll_input(maybe_current_dir: &Option<Direction>) -> Option<Direction> {
    let device_state = DeviceState::new();
    let current_keys = device_state.get_keys();
    for key in &current_keys {
        let maybe_dir = match key {
            device_query::keymap::Keycode::Up => Some(Direction::Up),
            device_query::keymap::Keycode::Down => Some(Direction::Down),
            device_query::keymap::Keycode::Left => Some(Direction::Left),
            device_query::keymap::Keycode::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(dir) = maybe_dir {
            if let Some(current_dir) = maybe_current_dir {
                if dir != *current_dir {
                    return maybe_dir;
                }
            } else {
                return maybe_dir;
            }
        }
    }
    None
}

pub fn tick(
    snake: &mut Snake,
    fruit_pos: &mut (i32, i32),
    keyboard: &mut RgbKeyboard,
    sound_manager: &mut SoundManager,
    maybe_dir: &mut Option<Direction>,
) -> bool {
    if let Some(new_direction) = maybe_dir {
        if !new_direction.is_opposite(snake.direction) {
            snake.direction = *new_direction;
        }
    }

    let maybe_tail_pos = snake.step(sound_manager);
    if let Some(tail_pos) = maybe_tail_pos {
        let tail_key = SNAKE_BOARD[tail_pos.1 as usize][tail_pos.0 as usize];
        utils::direct_set_key(keyboard, tail_key, BLACK);
        sound_manager.play(SoundType::Step);
    }
    if snake.touch_snek(*fruit_pos) {
        sound_manager.play(SoundType::Eat);
        move_fruit(fruit_pos, snake);
        snake.nom();
    }
    if snake.snek_is_ded() {
        sound_manager.play(SoundType::Death);
        snake.snek_go_boom(keyboard);
        return true;
    }
    draw_bounds(keyboard, WHITE * 0.1);
    let fruit_key = SNAKE_BOARD[fruit_pos.1 as usize][fruit_pos.0 as usize];
    utils::direct_set_key(keyboard, fruit_key, RED);
    snake.draw(keyboard);
    return false;
}

pub fn draw_bounds(keyboard: &mut RgbKeyboard, color: Color) {
    for key in BOARD_BOUNDS.iter() {
        utils::direct_set_key(keyboard, *key, color);
    }
}

pub fn move_fruit(fruit_pos: &mut (i32, i32), snek: &Snake) {
    let mut rng = rand::thread_rng();
    let max_iterations = 1000;
    for _i in 0..max_iterations {
        if !snek.touch_snek(*fruit_pos) {
            return;
        }
        fruit_pos.1 = rng.gen_range(0, SNAKE_BOARD.len() as i32);
        fruit_pos.0 = rng.gen_range(0, SNAKE_BOARD[0].len() as i32);
    }
}
