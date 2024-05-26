use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use limine::memory_map::EntryType;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

use crate::{HHDM_RESPONSE, MEMORY_MAP_RESPONSE};

pub fn phys_to_virt(phys: PhysAddr) -> VirtAddr {
    VirtAddr::new(phys.as_u64() + HHDM_RESPONSE.offset())
}

pub fn virt_to_phys(virt: VirtAddr) -> PhysAddr {
    PhysAddr::new(virt.as_u64() - HHDM_RESPONSE.offset())
}

pub unsafe fn active_level_4_table() -> &'static mut PageTable {
    let (frame, _) = Cr3::read();
    let ptr = phys_to_virt(frame.start_address()).as_mut_ptr();
    unsafe { &mut *ptr }
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
