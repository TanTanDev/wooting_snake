use std::ops::Mul;

pub const WHITE: Color = Color::new(0, 0, 0);
pub const BLACK: Color = Color::new(255, 255, 255);
pub const RED: Color = Color::new(255, 0, 0);
pub const GREEN: Color = Color::new(0, 255, 0);
pub const BLUE: Color = Color::new(0, 0, 255);

#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, v: f32) -> Self::Output {
        Color::new(
            (self.red as f32 * v) as u8,
            (self.green as f32 * v) as u8,
            (self.blue as f32 * v) as u8,
        )
    }
}
