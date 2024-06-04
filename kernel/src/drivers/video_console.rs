use core::{fmt, slice};

use spin::Lazy;

use crate::MODULE_RESPONSE;

use super::{
    display::{Color, Display},
    psf2::Font,
};

static FONT: Lazy<Font> = Lazy::new(|| {
    let file = MODULE_RESPONSE
        .modules()
        .iter()
        .find(|file| file.path().starts_with(b"/font/"))
        .expect("no font found");
    let addr = file.addr();
    let size = file.size() as usize;
    let slice = unsafe { slice::from_raw_parts(addr, size) };
    Font::parse(slice)
});

pub struct VideoConsole {
    display: Display,
    cursor_x: usize,
    cursor_y: usize,
    pub color_fg: Color,
    pub color_bg: Color,
}
unsafe impl Send for VideoConsole {}

impl VideoConsole {
    pub fn new(display: Display) -> Self {
        Self {
            display,
            cursor_x: 0,
            cursor_y: 0,
            color_fg: Color::new(255, 255, 255),
            color_bg: Color::new(0, 0, 0),
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
                            self.color_fg
                        } else {
                            self.color_bg
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
        self.display.scroll_down(FONT.height());
        self.cursor_y -= 1;
    }
}

impl fmt::Write for VideoConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
