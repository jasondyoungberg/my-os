use super::Color;

use bootloader_api::info::FrameBuffer;

pub struct PixelBuffer<'a> {
    buffer: &'a mut [u8],
    info: PixelBufferInfo,
}

pub struct PixelBufferInfo {
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
    pub stride: usize,
}

impl PixelBuffer<'_> {
    pub fn info(&self) -> &PixelBufferInfo {
        &self.info
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let offset = self.get_index(x, y);
        self.buffer[offset] = color.b;
        self.buffer[offset + 1] = color.g;
        self.buffer[offset + 2] = color.r;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let offset = self.get_index(x, y);
        Color {
            r: self.buffer[offset + 2],
            g: self.buffer[offset + 1],
            b: self.buffer[offset],
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        assert!(
            x < self.info.width,
            "x out of bounds ({} >= {})",
            x,
            self.info.width
        );
        assert!(
            y < self.info.height,
            "y out of bounds ({} >= {})",
            y,
            self.info.height
        );

        y * self.info.stride * self.info.bytes_per_pixel + x * self.info.bytes_per_pixel
    }

    pub fn clear(&mut self, color: &Color) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.set_pixel(x, y, color);
            }
        }
    }
}

impl From<FrameBuffer> for PixelBuffer<'_> {
    fn from(fb: FrameBuffer) -> Self {
        let info = fb.info();

        assert!(
            info.pixel_format == bootloader_api::info::PixelFormat::Bgr,
            "Only BGR pixel format is supported"
        );

        Self {
            buffer: fb.into_buffer(),
            info: PixelBufferInfo {
                width: info.width,
                height: info.height,
                bytes_per_pixel: info.bytes_per_pixel,
                stride: info.stride,
            },
        }
    }
}
