use crate::limine::RawFramebuffer;

pub struct Display {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bytes_per_pixel: usize,
    pub addr: *mut u8,
}

impl Display {
    pub fn new(framebuffer: &RawFramebuffer) -> Self {
        Self {
            width: framebuffer.width() as usize,
            height: framebuffer.height() as usize,
            pitch: framebuffer.pitch() as usize,
            bytes_per_pixel: framebuffer.bits_per_pixel() as usize / 8,
            addr: framebuffer.addr(),
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, color: u32) {
        let pixel_offset = y * self.pitch + x * self.bytes_per_pixel;
        unsafe { *(self.addr.add(pixel_offset) as *mut u32) = color };
    }

    pub fn clear(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }
}
