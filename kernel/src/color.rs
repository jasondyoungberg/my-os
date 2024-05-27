use core::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn blend(self, other: Self, alpha: f32) -> Self {
        self * (1.0 - alpha) + other * alpha
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            red: self.red.saturating_add(rhs.red),
            green: self.green.saturating_add(rhs.green),
            blue: self.blue.saturating_add(rhs.blue),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            red: (self.red as f32 * rhs) as u8,
            green: (self.green as f32 * rhs) as u8,
            blue: (self.blue as f32 * rhs) as u8,
        }
    }
}
