use limine::framebuffer::Framebuffer;

pub struct Display {
    width: usize,
    height: usize,

    bytes_per_pixel: usize,
    bytes_per_row: usize,
    total_bytes: usize,

    buffer: &'static mut [u8],
}

impl Display {
    pub fn new(framebuffer: &Framebuffer) -> Self {
        let width = framebuffer.width() as usize;
        let height = framebuffer.height() as usize;

        let bytes_per_pixel = framebuffer.bpp() as usize / 8;
        let bytes_per_row = framebuffer.pitch() as usize;
        let total_bytes = height * bytes_per_row;

        let buffer = unsafe { core::slice::from_raw_parts_mut(framebuffer.addr(), total_bytes) };

        Self {
            width,
            height,

            bytes_per_row,
            bytes_per_pixel,
            total_bytes,

            buffer,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let pixel_offset = y * self.bytes_per_row + x * self.bytes_per_pixel;

        self.buffer[pixel_offset] = color.b;
        self.buffer[pixel_offset + 1] = color.g;
        self.buffer[pixel_offset + 2] = color.r;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x >= self.width || y >= self.height {
            return Color::new(0, 0, 0);
        }

        let pixel_offset = y * self.bytes_per_row + x * self.bytes_per_pixel;

        Color::new(
            self.buffer[pixel_offset + 2],
            self.buffer[pixel_offset + 1],
            self.buffer[pixel_offset],
        )
    }

    // pub fn scroll(&mut self, x: isize, y: isize) {
    //     let offset = x * self.bytes_per_pixel as isize + y * self.bytes_per_row as isize;

    //     if offset < 0 {
    //         self.buffer.rotate_left(offset.unsigned_abs());
    //     } else {
    //         self.buffer.rotate_right(offset as usize);
    //     }
    // }

    pub fn scroll_down(&mut self, dist: usize) {
        self.buffer.copy_within(dist * self.bytes_per_row.., 0);
        self.buffer[self.total_bytes - dist * self.bytes_per_row..].fill(0);
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
