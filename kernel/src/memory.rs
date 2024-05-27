use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use limine::memory_map::EntryType;
use spin::Lazy;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{page_table::FrameError, FrameAllocator, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use crate::{HHDM_RESPONSE, MEMORY_MAP_RESPONSE};

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
