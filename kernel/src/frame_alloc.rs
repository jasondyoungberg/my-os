use core::sync::atomic::{AtomicU64, Ordering};

use limine::memory_map::EntryType;
use x86_64::{
    structures::paging::{self, PhysFrame, Size4KiB},
    PhysAddr,
};

use crate::requests::MEMORY_MAP_RESPONSE;

static NEXT: AtomicU64 = AtomicU64::new(0);
pub struct FrameAllocator;

/// Safety: This is safe because it never returns the same frame twice.
unsafe impl paging::FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<paging::PhysFrame<Size4KiB>> {
        let index = NEXT.fetch_add(1, Ordering::Relaxed);
        MEMORY_MAP_RESPONSE
            .entries()
            .iter()
            .filter(|&entry| entry.entry_type == EntryType::USABLE)
            .flat_map(|&entry| {
                PhysFrame::range(
                    PhysFrame::<Size4KiB>::from_start_address(PhysAddr::new(entry.base))
                        .expect("limine usable memory entries should be aligned"),
                    PhysFrame::from_start_address(PhysAddr::new(entry.base + entry.length))
                        .expect("limine usable memory entries should be aligned"),
                )
            })
            .nth(index as usize)
    }
}
