use x86_64::VirtAddr;

const MINISTACK_SIZE: usize = 64 * 1024; // 64 KB

#[repr(C, align(16))]
pub struct MiniStack {
    gaurd1: [u8; 4096],
    stack: [u8; MINISTACK_SIZE],
    gaurd2: [u8; 4096],
}

impl MiniStack {
    pub fn new() -> Self {
        Self {
            gaurd1: [0; 4096],
            stack: [0; MINISTACK_SIZE],
            gaurd2: [0; 4096],
        }
    }

    pub fn addr(&self) -> VirtAddr {
        VirtAddr::from_ptr(self.stack.as_ptr()) + MINISTACK_SIZE as u64
    }
}
