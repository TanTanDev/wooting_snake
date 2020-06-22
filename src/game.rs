use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use wooting_sdk::rgb::{RgbKeyboard};

use crate::color;
use crate::direction;
use crate::snake;
use crate::snake_board;
use crate::utils;

use color::*;
use snake::Snake;
use snake_board::*;
use direction::Direction;

pub fn run_game(keyboard: &mut RgbKeyboard){
    let mut snake = Snake::new();
    let mut fruit_pos = (7, 2);
    utils::clear(keyboard, BLACK);
    loop {
        let maybe_tail_pos = snake.step();
        if let Some(tail_pos) = maybe_tail_pos {
            let tail_key = SNAKE_BOARD[tail_pos.1 as usize][tail_pos.0 as usize];
            utils::direct_set_key(keyboard, tail_key, BLACK);
        }

        use device_query::{DeviceQuery, DeviceState};
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
            if let Some(new_direction) = maybe_dir {
                if !new_direction.is_opposite(snake.direction) {
                    snake.direction = new_direction;
                }
            }
        }
        if snake.touch_snek(&fruit_pos) {
            move_fruit(&mut fruit_pos, &mut snake);
            snake.nom();
        }
        if snake.snek_is_ded() {
            snake.snek_go_boom(keyboard);
            return;
        }
        draw_bounds(keyboard, WHITE * 0.1);
        let fruit_key = SNAKE_BOARD[fruit_pos.1 as usize][fruit_pos.0 as usize];
        utils::direct_set_key(keyboard, fruit_key, RED);
        snake.draw(keyboard);
        sleep(Duration::from_millis(100));
    }
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
        if !snek.touch_snek(fruit_pos) {
            return;
        }
        fruit_pos.1 = rng.gen_range(0, SNAKE_BOARD.len() as i32);
        fruit_pos.0 = rng.gen_range(0, SNAKE_BOARD[0].len() as i32);
    }
}