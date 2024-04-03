#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub const BLACK: Color = Color::new(0, 0, 0);
pub const RED: Color = Color::new(255, 0, 0);
pub const YELLOW: Color = Color::new(255, 255, 0);
pub const GREEN: Color = Color::new(0, 255, 0);
pub const CYAN: Color = Color::new(0, 255, 255);
pub const BLUE: Color = Color::new(0, 0, 255);
pub const MAGENTA: Color = Color::new(255, 0, 255);
pub const WHITE: Color = Color::new(255, 255, 255);
