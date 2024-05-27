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
}
