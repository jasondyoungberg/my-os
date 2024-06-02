use core::sync::atomic::AtomicUsize;

use limine::memory_map::EntryType;
use x86_64::{
    structures::paging::{FrameAllocator, FrameDeallocator, PhysFrame, Size4KiB},
    PhysAddr,
};

use crate::MEMORY_MAP_RESPONSE;

static NEXT: AtomicUsize = AtomicUsize::new(0);

pub struct MyFrameAllocator;
unsafe impl FrameAllocator<Size4KiB> for MyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        MEMORY_MAP_RESPONSE
            .entries()
            .iter()
            .filter(|entry| entry.entry_type == EntryType::USABLE)
            .flat_map(|entry| {
                let start_addr = PhysAddr::new(entry.base);
                let start_frame = PhysFrame::from_start_address(start_addr).unwrap();
                let end_frame = PhysFrame::from_start_address(start_addr + entry.length).unwrap();
                PhysFrame::range(start_frame, end_frame)
            })
            .nth(NEXT.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}

impl FrameDeallocator<Size4KiB> for MyFrameAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        let _ = frame;
        todo!()
    }
}
