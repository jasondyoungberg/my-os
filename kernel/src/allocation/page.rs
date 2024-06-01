use core::sync::atomic::AtomicU64;

use crate::{
    address::VirtAddr,
    structures::paging::{Page, PageRange},
};

pub static MMIO_ALLOCATOR: PageAllocator = PageAllocator::new(PageRange::new(
    Page::containing_addr(VirtAddr::new(0xffff_e000_0000_0000)),
    Page::containing_addr(VirtAddr::new(0xffff_f000_0000_0000)),
));

pub struct PageAllocator {
    next: AtomicU64,
    end: VirtAddr,
}
impl PageAllocator {
    pub const fn new(pages: PageRange) -> Self {
        let start = pages.start.start().as_u64();
        let end = pages.end.start();

        Self {
            next: AtomicU64::new(start),
            end,
        }
    }
    pub fn alloc(&self, pages: u64) -> Page {
        let next = self
            .next
            .fetch_add(pages * 4096, core::sync::atomic::Ordering::Relaxed);
        assert!(next + pages * 4096 < self.end.as_u64());
        Page::containing_addr(VirtAddr::new(next))
    }
}