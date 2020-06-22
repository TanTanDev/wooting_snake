use wooting_sdk::Key;
use wooting_sdk::Key::*;
pub static SNAKE_BOARD: &'static [[Key; 10]; 4] = &[
    [One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero],
    [Q, W, E, R, T, Y, U, I, O, P],
    [A, S, D, F, G, H, J, K, L, SemiColon],
    [Z, X, C, V, B, N, M, Comma, Period, ForwardSlash],
];

pub static BOARD_BOUNDS: [Key; 23] = [
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Dash,
    LeftBracket,
    Apostrophe,
    RightShift,
    Tilde,
    Tab,
    CapsLock,
    ISO2,
    LeftAlt,
    Space,
    RightAlt,
    RightMod,
    Fn,
];
