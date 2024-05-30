use core::ffi::c_void;

use super::{Response, MAGIC_1, MAGIC_2};

#[repr(C)]
pub struct FramebufferRequest {
    _id: [u64; 4],
    revision: u64,
    pub response: Response<FramebufferResponse>,
}
impl FramebufferRequest {
    pub const fn new() -> Self {
        Self {
            _id: [MAGIC_1, MAGIC_2, 0x9d5827dcd881dd75, 0xa3148604f6fab11b],
            revision: 0,
            response: Response::none(),
        }
    }
}

#[repr(C)]
pub struct FramebufferResponse {
    revision: u64,
    framebuffer_ct: u64,
    framebuffers: *const *const RawFramebuffer,
}
unsafe impl Sync for FramebufferResponse {}
unsafe impl Send for FramebufferResponse {}
impl FramebufferResponse {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn framebuffers(&self) -> impl Iterator<Item = &RawFramebuffer> {
        (unsafe { core::slice::from_raw_parts(self.framebuffers, self.framebuffer_ct as usize) })
            .iter()
            .map(|&fb| unsafe { &*fb })
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RawFramebuffer {
    addr: *mut c_void,
    width: u64,
    height: u64,
    pitch: u64,
    bits_per_pixel: u16,
    memory_model: u8,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
    _unused: [u8; 7],
    edid_size: u64,
    edid: *mut u8,
}

impl RawFramebuffer {
    pub fn addr(&self) -> *mut u8 {
        self.addr as *mut u8
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn pitch(&self) -> u64 {
        self.pitch
    }

    pub fn bits_per_pixel(&self) -> u16 {
        self.bits_per_pixel
    }

    pub fn memory_model(&self) -> u8 {
        self.memory_model
    }

    pub fn red_mask_size(&self) -> u8 {
        self.red_mask_size
    }

    pub fn red_mask_shift(&self) -> u8 {
        self.red_mask_shift
    }

    pub fn green_mask_size(&self) -> u8 {
        self.green_mask_size
    }

    pub fn green_mask_shift(&self) -> u8 {
        self.green_mask_shift
    }

    pub fn blue_mask_size(&self) -> u8 {
        self.blue_mask_size
    }

    pub fn blue_mask_shift(&self) -> u8 {
        self.blue_mask_shift
    }

    pub fn edid(&self) -> &[u8] {
        let ptr = self.edid as *const u8;
        let size = self.edid_size as usize;
        unsafe { core::slice::from_raw_parts(ptr, size) }
    }
}
