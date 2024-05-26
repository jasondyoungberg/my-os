use core::alloc::GlobalAlloc;

use linked_list_allocator::LockedHeap;
use spin::Lazy;
use x86_64::instructions::interrupts::without_interrupts;

const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

struct Allocator(Lazy<LockedHeap>);

impl Allocator {
    pub const fn new() -> Self {
        Self(Lazy::new(|| {
            static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
            unsafe { LockedHeap::new(HEAP.as_mut_ptr(), HEAP_SIZE) }
        }))
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let res = without_interrupts(|| unsafe { self.0.alloc(layout) });
        log::trace!(
            "alloc {:p}[{}] (align {})",
            res,
            layout.size(),
            layout.align()
        );
        res
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        without_interrupts(|| unsafe { self.0.dealloc(ptr, layout) });
        log::trace!(
            "dealloc {:p}[{}] (align {})",
            ptr,
            layout.size(),
            layout.align(),
        );
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let res = without_interrupts(|| unsafe { self.0.alloc_zeroed(layout) });
        log::trace!(
            "alloc {:p}[{}] (align {}) (zeroed)",
            res,
            layout.size(),
            layout.align()
        );
        res
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let res = without_interrupts(|| unsafe { self.0.realloc(ptr, layout, new_size) });
        log::trace!(
            "realloc {:p}[{}] -> {:p}[{}] (align {})",
            ptr,
            layout.size(),
            res,
            new_size,
            layout.align()
        );
        res
    }
}
