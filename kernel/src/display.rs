use crate::graphics::{color::Color, Drawable};

use bootloader_api::info::FrameBuffer;
use spin::{Mutex, Once};

pub static DISPLAY: Once<Mutex<FrameBuffer>> = Once::new();

pub fn init(framebuffer: FrameBuffer) {
    DISPLAY.call_once(|| Mutex::new(framebuffer));
}

impl Drawable for FrameBuffer {
    fn size(&self) -> (usize, usize) {
        let info = self.info();

        (info.width, info.height)
    }

    fn set_pixel_unchecked(&mut self, pos: (usize, usize), color: Color) {
        let info = self.info();
        let index = (pos.1 * info.stride + pos.0) * info.bytes_per_pixel;
        let buffer = self.buffer_mut();

        match info.pixel_format {
            bootloader_api::info::PixelFormat::Rgb => {
                buffer[index] = color.r;
                buffer[index + 1] = color.g;
                buffer[index + 2] = color.b;
            }
            bootloader_api::info::PixelFormat::Bgr => {
                buffer[index] = color.b;
                buffer[index + 1] = color.g;
                buffer[index + 2] = color.r;
            }
            _ => todo!(),
        }
    }
}
