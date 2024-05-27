use core::{fmt::Debug, slice};

use alloc::{vec, vec::Vec};

use crate::color::Color;

pub struct FrameBuffer<'a> {
    data: &'a mut [u8],
    backbuffer: Vec<u8>,
    pub width: u64,
    pub height: u64,
    pitch: u64,
    size: usize,
    format: PixelFormat,
}

#[derive(Debug)]
enum PixelFormat {
    Rgb24,
    Rgb32,
    Bgr24,
    Bgr32,
}

impl FrameBuffer<'_> {
    pub fn set_pixel(&mut self, pos: (u64, u64), color: Color) {
        let (col, row) = pos;

        assert!(col < self.width, "x out of bounds");
        assert!(row < self.height, "y out of bounds");

        match self.format {
            PixelFormat::Bgr24 => {
                let offset = (row * self.pitch + col * 3) as usize;
                self.backbuffer[offset] = color.blue;
                self.backbuffer[offset + 1] = color.green;
                self.backbuffer[offset + 2] = color.red;
            }
            PixelFormat::Rgb24 => {
                let offset = (row * self.pitch + col * 3) as usize;
                self.backbuffer[offset] = color.red;
                self.backbuffer[offset + 1] = color.green;
                self.backbuffer[offset + 2] = color.blue;
            }
            PixelFormat::Bgr32 => {
                let offset = (row * self.pitch + col * 4) as usize;
                self.backbuffer[offset] = color.blue;
                self.backbuffer[offset + 1] = color.green;
                self.backbuffer[offset + 2] = color.red;
            }
            PixelFormat::Rgb32 => {
                let offset = (row * self.pitch + col * 4) as usize;
                self.backbuffer[offset] = color.red;
                self.backbuffer[offset + 1] = color.green;
                self.backbuffer[offset + 2] = color.blue;
            }
        }
    }

    pub fn scroll(&mut self, offset: (i64, i64)) {
        let (dx, dy) = offset;

        let shift = dy * self.pitch as i64 + dx * self.format.bytes() as i64;
        let shift = isize::try_from(shift).unwrap();

        if shift > 0 {
            self.backbuffer
                .rotate_right(usize::try_from(shift).unwrap());
        } else {
            self.backbuffer
                .rotate_left(usize::try_from(-shift).unwrap());
        }
    }

    pub fn flush(&mut self) {
        let source: &[u8] = &self.backbuffer;
        let dest: &mut [u8] = self.data;

        dest.copy_from_slice(source);
    }
}

impl PixelFormat {
    const fn bytes(&self) -> u8 {
        match self {
            Self::Rgb24 | Self::Bgr24 => 3,
            Self::Rgb32 | Self::Bgr32 => 4,
        }
    }
}

impl From<limine::framebuffer::Framebuffer<'_>> for FrameBuffer<'_> {
    fn from(fb: limine::framebuffer::Framebuffer) -> Self {
        assert_eq!(fb.red_mask_size(), 8, "Display must be 8-bit RGB");
        assert_eq!(fb.green_mask_size(), 8, "Display must be 8-bit RGB");
        assert_eq!(fb.blue_mask_size(), 8, "Display must be 8-bit RGB");
        assert_eq!(fb.bpp() % 8, 0, "Display must be byte-aligned");

        let format = match (
            fb.bpp(),
            fb.red_mask_shift(),
            fb.green_mask_shift(),
            fb.blue_mask_shift(),
        ) {
            (24, 0, 8, 16) => PixelFormat::Rgb24,
            (32, 0, 8, 16) => PixelFormat::Rgb32,
            (24, 16, 8, 0) => PixelFormat::Bgr24,
            (32, 16, 8, 0) => PixelFormat::Bgr32,
            _ => panic!("Unsupported pixel format"),
        };

        let size = usize::try_from(fb.height() * fb.pitch()).unwrap();

        let slice = unsafe { slice::from_raw_parts_mut(fb.addr(), size) };

        Self {
            data: slice,
            backbuffer: vec![0; size],
            width: fb.width(),
            height: fb.height(),
            pitch: fb.pitch(),
            size,
            format,
        }
    }
}

impl Debug for FrameBuffer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FrameBuffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("pitch", &self.pitch)
            .field("size", &self.size)
            .field("format", &self.format)
            .finish()
    }
}
