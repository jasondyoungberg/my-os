use core::sync::atomic::AtomicUsize;

use crate::{
    limine::MemoryMapEntryType,
    structures::paging::{PhysFrame, PhysFrameRange},
    MEMORY_MAP_REQUEST,
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

pub fn alloc_frame() -> PhysFrame {
    MEMORY_MAP_REQUEST
        .response
        .get()
        .unwrap()
        .entries()
        .filter(|entry| entry.entry_type == MemoryMapEntryType::Usable)
        .flat_map(|entry| {
            let start = PhysFrame::containing_addr(entry.base);
            let end = PhysFrame::containing_addr(entry.base + entry.length);
            PhysFrameRange::new(start, end)
        })
        .nth(NEXT.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
        .unwrap()
}

pub fn dealloc_frame(_frame: PhysFrame) {
    // todo
}
