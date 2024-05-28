use x86_64::{
    structures::paging::{PageTableFlags, PhysFrame},
    PhysAddr, VirtAddr,
};

use crate::memory::{map_kernel_frame_to_page, map_kernel_page, PageAllocator};

static MINISTACK_ALLOCATOR: PageAllocator =
    PageAllocator::new(0xFFFF_9000_0000_1000, 0xFFFF_9000_FFFF_F000);
static MMIO_ALLOCATOR: PageAllocator =
    PageAllocator::new(0xFFFF_9001_0000_0000, 0xFFFF_9001_FFFF_F000);

pub fn create_ministack(size: u64) -> VirtAddr {
    let range = MINISTACK_ALLOCATOR.alloc_range(size + 4096);
    let range = range.start..range.end - 1;
    let stack_pointer = range.end.start_address();

    for page in range {
        map_kernel_page(page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    }

    stack_pointer
}

pub fn map_mmio(addr: u64, size: u64) -> VirtAddr {
    let range = MMIO_ALLOCATOR.alloc_range(size);
    let virt_addr = range.start.start_address();

    crate::dbg!(&range);

    for (i, page) in range.enumerate() {
        let frame = PhysFrame::containing_address(PhysAddr::new(i as u64 * 4096 + addr));
        map_kernel_frame_to_page(
            frame,
            page,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE,
        );
    }

    virt_addr
}
