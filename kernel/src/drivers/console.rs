use core::fmt;

use super::display::Display;

pub struct Console {
    display: Display,
    cursor_x: usize,
    cursor_y: usize,
}
unsafe impl Send for Console {}

impl Console {
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
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            ' '..='~' => {
                let ascii = c as u8;
                let data_index = (ascii - 0x20) as usize * FONT_WIDTH * FONT_HEIGHT;
                for y in 0..FONT_HEIGHT {
                    for x in 0..FONT_WIDTH {
                        let gray = FONT_DATA[data_index + y * FONT_WIDTH + x] as u32;

                        let color = gray << 16 | gray << 8 | gray;

                        self.display.set_pixel(
                            self.cursor_x * FONT_WIDTH + x,
                            self.cursor_y * FONT_HEIGHT + y,
                            color,
                        );
                    }
                }
                self.cursor_x += 1;
            }
            _ => panic!("disallowed character"),
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

const FONT_DATA: &[u8; 95 * FONT_WIDTH * FONT_HEIGHT] = include_bytes!("font.bin");
const FONT_WIDTH: usize = 16;
const FONT_HEIGHT: usize = 32;
