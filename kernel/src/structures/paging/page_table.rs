use core::{fmt, ops};

use crate::{address::PhysAddr, allocation::frame::alloc_frame};

use super::frame::PhysFrame;

#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}
impl PageTable {
    pub fn from_frame(frame: PhysFrame) -> &'static mut Self {
        let addr = frame.start().to_virt().as_mut_ptr::<Self>();
        unsafe { &mut *addr }
    }
    pub fn set_empty(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.entries.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.entries.iter_mut()
    }

    pub fn next_table(&mut self, index: usize, flags: PageTableFlags) -> &mut PageTable {
        let entry = &mut self.entries[index];
        if entry.is_unused() {
            let frame = alloc_frame();
            let new_table = PageTable::from_frame(frame);
            new_table.set_empty();
            entry.set_frame(frame);
        }
        entry.set_flags(entry.flags() | flags);
        Self::from_frame(entry.frame())
    }
}
impl fmt::Debug for PageTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.entries.iter().enumerate())
            .finish()
    }
}
impl ops::Index<usize> for PageTable {
    type Output = PageTableEntry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}
impl ops::IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        const NO_EXECUTE =      1 << 63;
    }
}

const ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageTableEntry(u64);
impl PageTableEntry {
    pub fn new() -> Self {
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
        PhysFrame::try_from(PhysAddr::new(self.0 & ADDR_MASK)).unwrap()
    }
    pub fn set_frame(&mut self, frame: PhysFrame) {
        assert_eq!(frame.start().as_u64() & !ADDR_MASK, 0);
        self.0 = frame.start().as_u64() | self.flags().bits();
    }
}
impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_unused() {
            write!(f, "PageTableEntry {{ empty }}")
        } else {
            f.debug_struct("PageTableEntry")
                .field("frame", &self.frame())
                .field("flags", &self.flags())
                .finish()
        }
    }
}
