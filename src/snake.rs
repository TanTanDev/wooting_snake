use crate::color;
use crate::direction;
use crate::snake_board;
use crate::utils;

use wooting_sdk::rgb::{RgbKeyboard};

use color::Color;
use direction::Direction;
use snake_board::SNAKE_BOARD;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

static START_POSITION: (i32, i32) = (3, 2);
static POSITIONS_CAPACITY: usize =
    snake_board::SNAKE_BOARD.len() * snake_board::SNAKE_BOARD[0].len();

pub struct Snake {
    positions: VecDeque<(i32, i32)>,
    pub direction: Direction,
    color: Color,
    head_color: Color,
    length: i32,
}

impl Snake {
    pub fn new() -> Self {
        let mut positions = VecDeque::with_capacity(POSITIONS_CAPACITY);
        positions.push_back(START_POSITION);
        Snake {
            positions,
            direction: Direction::Right,
            color: color::BLUE,
            head_color: color::GREEN,
            length: 3,
        }
    }

    pub fn nom(&mut self) {
        self.length += 1;
    }

    pub fn touch_snek(&self, pos: &(i32, i32)) -> bool {
        for (x, y) in self.positions.iter() {
            if *x == pos.0 && *y == pos.1 {
                return true;
            }
        }
        false
    }

    pub fn snek_is_ded(&mut self) -> bool {
        let maybe_back = self.positions.back();
        if let Some(&(snek_head_x, snek_head_y)) = maybe_back {
            for (snek_body_x, snek_body_y) in self.positions.iter().rev().skip(1) {
                if *snek_body_x == snek_head_x && *snek_body_y == snek_head_y {
                    return true;
                }
            }
        }
        false
    }

    pub fn snek_go_boom(&mut self, keyboard: &mut RgbKeyboard) {
        sleep(Duration::from_millis(400));
        for (x, y) in self.positions.iter().rev() {
            let key = SNAKE_BOARD[*y as usize][*x as usize];
            utils::direct_set_key(keyboard, key, color::RED);
            sleep(Duration::from_millis(50));
            utils::direct_set_key(keyboard, key, color::WHITE);
            sleep(Duration::from_millis(50));
        }
        for (x, y) in self.positions.iter() {
            let key = SNAKE_BOARD[*y as usize][*x as usize];
            utils::direct_set_key(keyboard, key, color::RED);
            sleep(Duration::from_millis(50));
            utils::direct_set_key(keyboard, key, color::WHITE);
            sleep(Duration::from_millis(20));
        }
        utils::clear(keyboard, color::RED);
        sleep(Duration::from_millis(200));
        utils::clear(keyboard, color::BLACK);
        sleep(Duration::from_millis(40));
        utils::clear(keyboard, color::RED);
        sleep(Duration::from_millis(200));
    }

    // Returns the tail that was removed
    pub fn step(&mut self) -> Option<(i32, i32)> {
        let maybe_head_pos = self.positions.back();
        let mut last_head_pos = (0, 0);
        if let Some(head_pos) = maybe_head_pos {
            last_head_pos = *head_pos;
        }
        let velocity = self.direction.velocity();
        let mut next_pos = (last_head_pos.0 + velocity.0, last_head_pos.1 + velocity.1);

        // Keep inside bounds
        let board_height = SNAKE_BOARD.len() as i32;
        if next_pos.1 >= board_height {
            next_pos.1 = 0;
        } else if next_pos.1 < 0 {
            next_pos.1 = board_height - 1;
        }
        let board_width = SNAKE_BOARD[0].len() as i32;
        if next_pos.0 >= board_width {
            next_pos.0 = 0;
        } else if next_pos.0 < 0 {
            next_pos.0 = board_width - 1;
        }
        self.positions.push_back(next_pos);
        if self.positions.len() > self.length as usize {
            return self.positions.pop_front();
        }
        None
    }

    pub fn draw(&self, keyboard: &mut RgbKeyboard) {
        for (i, (x, y)) in self.positions.iter().enumerate() {
            let key = SNAKE_BOARD[*y as usize][*x as usize];
            let color;
            if i == self.positions.len() - 1 {
                color = self.head_color;
            } else {
                color = self.color;
            }
            utils::direct_set_key(keyboard, key, color);
        }
    }
}
