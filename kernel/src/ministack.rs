use core::sync::atomic::AtomicU64;

use x86_64::{
    structures::paging::{
        page::PageRange, FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags,
    },
    VirtAddr,
};

use crate::{
    memory::{MemoryMapFrameAllocator, MEMORY_OFFSET},
    process::MANAGER,
};

const RANGE_START: u64 = 0xFFFF_9000_0000_0000 + 4096;
const RANGE_END: u64 = 0xFFFF_9000_FFFF_FFFF - 4096;

static MINISTACK_ADDR: AtomicU64 = AtomicU64::new(RANGE_START);

pub fn create_ministack(size: u64) -> VirtAddr {
    let mut manager = MANAGER.get().unwrap().lock();
    let l4_table = manager.kernel_l4_table_mut();
    let mut mapper = unsafe { OffsetPageTable::new(l4_table, *MEMORY_OFFSET) };
    let mut frame_allocator = MemoryMapFrameAllocator;

    let start_addr =
        VirtAddr::new(MINISTACK_ADDR.fetch_add(size + 4096, core::sync::atomic::Ordering::Relaxed));
    let end_addr = start_addr + size;

    assert!(
        end_addr.as_u64() <= RANGE_END,
        "Ministack address space is empty"
    );

    let range: PageRange = PageRange {
        start: Page::containing_address(start_addr),
        end: Page::containing_address(end_addr),
    };

    log::debug!("Ministack at {:?}", range);

    for page in range {
        let frame = frame_allocator
            .allocate_frame()
            .expect("Failed to allocate frame");
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator) }
            .expect("Failed to map page")
            .flush();
    }

    end_addr - 16
}
