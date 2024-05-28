use x86_64::{structures::paging::PageTableFlags, VirtAddr};

use crate::{memory::map_kernel_page, memory::PageAllocator};

static PAGE_ALLOCATOR: PageAllocator =
    PageAllocator::new(0xFFFF_9000_0000_1000, 0xFFFF_9000_FFFF_F000);

pub fn create_ministack(size: u64) -> VirtAddr {
    let range = PAGE_ALLOCATOR.alloc_range(size + 4096);
    let range = range.start..range.end - 1;
    let stack_pointer = range.end.start_address();

    for page in range {
        map_kernel_page(page, PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    }

    stack_pointer
}
