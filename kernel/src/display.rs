use crate::graphics::PixelBuffer;

use bootloader_api::info::FrameBuffer;
use spin::{Mutex, Once};

pub static DISPLAY: Once<Mutex<FrameBuffer>> = Once::new();

pub fn init(framebuffer: FrameBuffer) {
    DISPLAY.call_once(|| Mutex::new(framebuffer));
}

impl PixelBuffer for FrameBuffer {
    fn set_pixel(&mut self, pos: (usize, usize), color: &crate::graphics::Color) {
        let info = self.info();
        let buffer = self.buffer_mut();
        let offset = (pos.1 * info.stride + pos.0) * info.bytes_per_pixel;

        match info.pixel_format {
            bootloader_api::info::PixelFormat::Rgb => {
                buffer[offset] = color.r;
                buffer[offset + 1] = color.g;
                buffer[offset + 2] = color.b;
            }
            bootloader_api::info::PixelFormat::Bgr => {
                buffer[offset] = color.b;
                buffer[offset + 1] = color.g;
                buffer[offset + 2] = color.r;
            }
            _ => todo!(),
        }
    }

    fn get_pixel(&self, pos: (usize, usize)) -> crate::graphics::Color {
        let info = self.info();
        let buffer = self.buffer();
        let offset = (pos.1 * info.stride + pos.0) * info.bytes_per_pixel;

        match info.pixel_format {
            bootloader_api::info::PixelFormat::Rgb => crate::graphics::Color {
                r: buffer[offset],
                g: buffer[offset + 1],
                b: buffer[offset + 2],
            },
            bootloader_api::info::PixelFormat::Bgr => crate::graphics::Color {
                r: buffer[offset + 2],
                g: buffer[offset + 1],
                b: buffer[offset],
            },
            _ => todo!(),
        }
    }

    fn size(&self) -> (usize, usize) {
        let info = self.info();
        (info.width, info.height)
    }
}
