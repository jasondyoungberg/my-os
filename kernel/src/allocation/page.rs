use core::sync::atomic::AtomicU64;

use x86_64::{
    structures::paging::{page::PageRange, Page},
    VirtAddr,
};

pub static STACK_ALLOCATOR: PageAllocator = PageAllocator::new(Page::range(
    Page::containing_address(VirtAddr::new(0xffff_d000_0000_0000)),
    Page::containing_address(VirtAddr::new(0xffff_e000_0000_0000)),
));
pub static MMIO_ALLOCATOR: PageAllocator = PageAllocator::new(Page::range(
    Page::containing_address(VirtAddr::new(0xffff_e000_0000_0000)),
    Page::containing_address(VirtAddr::new(0xffff_f000_0000_0000)),
));

pub struct PageAllocator {
    next: AtomicU64,
    end: VirtAddr,
}
impl PageAllocator {
    pub const fn new(pages: PageRange) -> Self {
        let start = pages.start.start_address().as_u64();
        let end = pages.end.start_address();

        Self {
            next: AtomicU64::new(start),
            end,
        }
    }
    pub fn alloc(&self) -> Page {
        let next = self
            .next
            .fetch_add(4096, core::sync::atomic::Ordering::Relaxed);
        assert!(next + 4096 < self.end.as_u64());
        Page::containing_address(VirtAddr::new(next))
    }
}
