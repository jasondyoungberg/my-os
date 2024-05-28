use core::sync::atomic::{
    AtomicU64, AtomicUsize,
    Ordering::{self, Relaxed},
};

use limine::memory_map::EntryType;
use spin::Lazy;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        page::PageRange, page_table::FrameError, FrameAllocator, Mapper, OffsetPageTable, Page,
        PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::{process::MANAGER, HHDM_RESPONSE, MEMORY_MAP_RESPONSE};

pub static MEMORY_OFFSET: Lazy<VirtAddr> = Lazy::new(|| VirtAddr::new(HHDM_RESPONSE.offset()));

pub fn phys_to_virt(phys: PhysAddr) -> VirtAddr {
    VirtAddr::new(phys.as_u64() + MEMORY_OFFSET.as_u64())
}

// code from https://os.phil-opp.com/paging-implementation/#translating-addresses
pub fn virt_to_phys(addr: VirtAddr) -> Option<PhysAddr> {
    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = *MEMORY_OFFSET + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub struct MemoryMapFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for MemoryMapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        MEMORY_MAP_RESPONSE
            .entries()
            .iter()
            .filter(|e| e.entry_type == EntryType::USABLE)
            .map(|e| e.base..e.base + e.length)
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
            .nth(NEXT.fetch_add(1, Relaxed))
    }
}
pub struct PageAllocator {
    start: u64,
    end: u64,
    position: AtomicU64,
}

impl PageAllocator {
    pub const fn new(start: u64, end: u64) -> Self {
        assert!(start % 4096 == 0);
        Self {
            start,
            end,
            position: AtomicU64::new(start),
        }
    }

    pub fn alloc(&self) -> Page {
        let addr = self.position.fetch_add(4096, Ordering::Relaxed);
        assert!(addr < self.end, "Out of memory");
        Page::containing_address(VirtAddr::new(addr))
    }

    pub fn alloc_range(&self, size: u64) -> PageRange {
        let size = (size + 4095) / 4096 * 4096;

        let start_addr = self.position.fetch_add(size, Ordering::Relaxed);
        let end_addr = start_addr + size;

        assert!(end_addr < self.end, "Out of memory");

        PageRange {
            start: Page::containing_address(VirtAddr::new(start_addr)),
            end: Page::containing_address(VirtAddr::new(end_addr)),
        }
    }
}

pub fn map_kernel_page(page: Page<Size4KiB>, flags: PageTableFlags) -> PhysFrame {
    let manager = MANAGER.get().unwrap().lock();
    let l4_table = manager.get_kernel_l4_table();
    let mut l4_table = l4_table.lock();
    map_page(page, flags, &mut l4_table)
}

pub fn map_kernel_frame_to_page(
    frame: PhysFrame,
    page: Page<Size4KiB>,
    flags: PageTableFlags,
) -> PhysFrame {
    let manager = MANAGER.get().unwrap().lock();
    let l4_table = manager.get_kernel_l4_table();
    let mut l4_table = l4_table.lock();
    map_frame_to_page(frame, page, flags, &mut l4_table)
}

pub fn map_page(
    page: Page<Size4KiB>,
    flags: PageTableFlags,
    l4_table: &mut PageTable,
) -> PhysFrame {
    let mut mapper = unsafe { OffsetPageTable::new(l4_table, *MEMORY_OFFSET) };
    let mut frame_allocator = MemoryMapFrameAllocator;
    let frame = frame_allocator.allocate_frame().expect("Out of memory");

    unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator) }
        .unwrap()
        .flush();

    frame
}

pub fn map_frame_to_page(
    frame: PhysFrame,
    page: Page<Size4KiB>,
    flags: PageTableFlags,
    l4_table: &mut PageTable,
) -> PhysFrame {
    let mut mapper = unsafe { OffsetPageTable::new(l4_table, *MEMORY_OFFSET) };
    let mut frame_allocator = MemoryMapFrameAllocator;

    unsafe { mapper.map_to(page, frame, flags, &mut frame_allocator) }
        .unwrap()
        .flush();

    frame
}
