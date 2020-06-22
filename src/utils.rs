use crate::all_keys::ALL_KEYS;
use crate::color;
use color::*;
use wooting_sdk::rgb::{RgbKeyboard};
use wooting_sdk::Key;

pub fn direct_set_key(keyboard: &mut RgbKeyboard, key: Key, color: Color) {
    keyboard.direct_set_key(key, color.red, color.green, color.blue);
}

pub fn array_set_single(keyboard: &mut RgbKeyboard, key: Key, color: Color) {
    keyboard.array_set_single(key, color.red, color.green, color.blue);
}

pub fn clear(keyboard: &mut RgbKeyboard, color: Color) {
    for key in ALL_KEYS {
        array_set_single(keyboard, *key, color);
    }
    keyboard.array_update();
}

pub fn column(keyboard: &mut RgbKeyboard, row_index: usize, color: Color) {
    let start = row_index * 16;
    let end = start + 16;
    for key in ALL_KEYS[start..end].iter() {
        array_set_single(keyboard, *key, color);
    }

    keyboard.array_update();
}
