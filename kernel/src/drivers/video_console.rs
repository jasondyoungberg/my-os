use core::fmt;

use spin::Lazy;

use crate::MODULE_REQUEST;

use super::{
    display::{Color, Display},
    psf2::Font,
};

static FONT: Lazy<Font> = Lazy::new(|| {
    let file = MODULE_REQUEST
        .response
        .get()
        .unwrap()
        .modules()
        .iter()
        .find(|file| file.path().starts_with("/font/"))
        .unwrap();
    Font::parse(file.slice())
});

pub struct VideoConsole {
    display: Display,
    cursor_x: usize,
    cursor_y: usize,
}
unsafe impl Send for VideoConsole {}

impl VideoConsole {
    pub fn new(display: Display) -> Self {
        Self {
            display,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\r' => {
                self.cursor_x = 0;
            }
            '\n' => {
                self.newline();
            }
            '\t' => {
                self.cursor_x = (self.cursor_x + 4) & !3;
            }
            c => {
                let glyph = FONT.get_char_fallback(c);
                for y in 0..FONT.height() {
                    for x in 0..FONT.width() {
                        let row_index = y * ((FONT.width() + 7) / 8);
                        let byte_index = row_index + x / 8;
                        let bit_index = 7 - (x % 8);
                        let bit = glyph[byte_index] >> bit_index & 1;

                        let color = if bit == 1 {
                            Color::new(0, 128, 255)
                        } else {
                            Color::new(0, 0, 0)
                        };

                        self.display.set_pixel(
                            self.cursor_x * FONT.width() + x,
                            self.cursor_y * FONT.height() + y,
                            color,
                        );
                    }
                }
                self.cursor_x += 1;
            }
        }

        if self.cursor_x * FONT.width() >= self.display.width() {
            self.newline();
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y * FONT.height() >= self.display.height() {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        self.display.scroll(0, -(FONT.height() as isize));
        self.cursor_y -= 1;

        let clear_start = self.cursor_y * FONT.height();
        let clear_end = self.display.height();
        for y in clear_start..clear_end {
            for x in 0..self.display.width() {
                self.display.set_pixel(x, y, Color::new(0, 0, 0));
            }
        }
    }
}

impl fmt::Write for VideoConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
