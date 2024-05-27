use core::fmt::{self, Write};

use noto_sans_mono_bitmap::{get_raster_width, FontWeight, RasterHeight, RasterizedChar};
use spin::{Lazy, Mutex};

const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const FONT_SIZE: RasterHeight = RasterHeight::Size32;
const FONT_WIDTH: usize = get_raster_width(FONT_WEIGHT, FONT_SIZE);
const FONT_HEIGHT: usize = FONT_SIZE.val();

use crate::{color::Color, display::FrameBuffer, FRAMEBUFFER_RESPONSE};

pub static CONSOLE: Lazy<Mutex<Console<'static>>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_RESPONSE
        .framebuffers()
        .next()
        .expect("no framebuffers found");

    Mutex::new(Console::new(framebuffer.into()))
});

pub struct Console<'a> {
    framebuffer: FrameBuffer<'a>,
    cursor: u64,
    foreground: Color,
    background: Color,
}

impl<'a> Console<'a> {
    const fn new(framebuffer: FrameBuffer<'a>) -> Self {
        Self {
            framebuffer,
            cursor: 0,
            foreground: Color::WHITE,
            background: Color::BLACK,
        }
    }

    fn flush(&mut self) {
        self.framebuffer.flush();
    }

    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    fn write_char(&mut self, c: char) {
        if c == '\n' || self.cursor >= self.framebuffer.width / FONT_WIDTH as u64 {
            self.newline();
            return;
        }

        let rasterized_char = Self::get_raster(c);
        let raster = rasterized_char.raster();

        for (y, row) in raster.iter().enumerate() {
            for (x, pix) in row.iter().enumerate() {
                let color = Color::blend(self.background, self.foreground, *pix as f32 / 255.0);

                let pix_x = (self.cursor as usize * FONT_WIDTH + x) as u64;
                let pix_y = self.framebuffer.height - FONT_HEIGHT as u64 + y as u64;

                self.framebuffer.set_pixel((pix_x, pix_y), color);
            }
        }

        self.cursor += 1;
    }

    fn get_raster(c: char) -> RasterizedChar {
        noto_sans_mono_bitmap::get_raster(c, FONT_WEIGHT, FONT_SIZE)
            .or(noto_sans_mono_bitmap::get_raster(
                '\u{FFFD}',
                FONT_WEIGHT,
                FONT_SIZE,
            ))
            .expect("fallback raster not found")
    }

    fn newline(&mut self) {
        self.framebuffer.scroll((0, -(FONT_HEIGHT as i64)));
        self.cursor = 0;

        for y in 0..FONT_HEIGHT {
            for x in 0..self.framebuffer.width {
                self.framebuffer.set_pixel(
                    (x, self.framebuffer.height - FONT_HEIGHT as u64 + y as u64),
                    Color::BLACK,
                );
            }
        }
    }
}

impl Write for Console<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut console = CONSOLE.lock();
    console.write_fmt(args).unwrap();
    console.flush();
}
