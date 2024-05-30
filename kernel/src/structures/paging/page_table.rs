use crate::address::PhysAddr;

use super::frame::PhysFrame;

#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct PageTableFlags: u64 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
    }
}

const ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

pub struct PageTableEntry(u64);
impl PageTableEntry {
    pub fn empty() -> Self {
        PageTableEntry(0)
    }
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_truncate(self.0)
    }

    pub fn set_flags(&mut self, flags: PageTableFlags) {
        self.0 = (self.0 & ADDR_MASK) | flags.bits()
    }

    pub fn frame(&self) -> PhysFrame {
        PhysFrame::new(PhysAddr::new(self.0 & ADDR_MASK))
    }

    pub fn set_frame(&mut self, frame: PhysFrame) {
        assert_eq!(frame.start().as_u64() & !ADDR_MASK, 0);
        self.0 = frame.start().as_u64() | self.flags().bits();
    }
}
