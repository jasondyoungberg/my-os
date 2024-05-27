use core::{fmt::Debug, slice};

use alloc::{vec, vec::Vec};
use spin::{Lazy, Mutex};

use crate::FRAMEBUFFER_RESPONSE;

pub static FRAMEBUFFER: Lazy<Mutex<FrameBuffer>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_RESPONSE
        .framebuffers()
        .next()
        .expect("no framebuffers found");

    Mutex::new(framebuffer.into())
});

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
pub enum PixelFormat {
    Rgb24,
    Rgb32,
    Bgr24,
    Bgr32,
}

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl FrameBuffer<'_> {
    pub fn set_pixel(&mut self, pos: (u64, u64), color: Color) {
        let (col, row) = pos;

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

    pub fn flush(&mut self) {
        let source: &[u8] = &self.backbuffer;
        let dest: &mut [u8] = self.data;

        dest.copy_from_slice(source);
    }
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
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
