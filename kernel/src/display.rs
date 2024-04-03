use crate::graphics::PixelBuffer;

use bootloader_api::info::FrameBuffer;
use spin::{Mutex, Once};

pub static DISPLAY: Once<Mutex<PixelBuffer>> = Once::new();

pub fn init(framebuffer: FrameBuffer) {
    DISPLAY.call_once(|| Mutex::new(PixelBuffer::from(framebuffer)));
}
