use spin::{Lazy, Mutex};

use crate::{address::VirtAddr, instructions};

const HEAP_SIZE: usize = 256 * 1024 * 1024;

#[cfg_attr(not(test), global_allocator)]
static GLOBAL_ALLOC: GlobalAlloc = GlobalAlloc;

static HEAP: Lazy<Mutex<Heap>> = Lazy::new(|| {
    static mut DATA: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

    Mutex::new(Heap {
        start: VirtAddr::from_mut_ptr(unsafe { DATA.as_mut_ptr() }),
        size: unsafe { DATA.len() },
        next: VirtAddr::from_mut_ptr(unsafe { DATA.as_mut_ptr() }),
    })
});

struct GlobalAlloc;
unsafe impl core::alloc::GlobalAlloc for GlobalAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        instructions::without_interrupts(|| {
            let mut heap = HEAP.lock();
            let start = heap.next.align_up(layout.align() as u64);
            let end = start + layout.size() as u64;

            if end > heap.start + heap.size as u64 {
                return core::ptr::null_mut();
            }

            heap.next = end;

            start.as_mut_ptr::<u8>()
        })
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // todo
    }
}

struct Heap {
    start: VirtAddr,
    size: usize,
    next: VirtAddr,
}
