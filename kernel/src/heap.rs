use core::ptr::addr_of;

use spin::{Lazy, Mutex};

use crate::instructions;

const HEAP_SIZE: usize = 256 * 1024 * 1024;
static mut HEAP_DATA: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[cfg_attr(not(test), global_allocator)]
static HEAP: Heap = Heap::new();

struct Heap {
    data: Lazy<Mutex<HeapData>>,
}
impl Heap {
    const fn new() -> Self {
        Self {
            data: Lazy::new(|| {
                Mutex::new(HeapData {
                    start: unsafe { addr_of!(HEAP_DATA) as usize },
                    size: 4096,
                    next: unsafe { addr_of!(HEAP_DATA) as usize },
                })
            }),
        }
    }
}
unsafe impl core::alloc::GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        instructions::without_interrupts(|| {
            let mut data = self.data.lock();
            let start = align_up(data.next, layout.align());
            let end = start + layout.size();

            if end > data.start + data.size {
                return core::ptr::null_mut();
            }

            data.next = end;

            start as *mut u8
        })
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

struct HeapData {
    start: usize,
    size: usize,
    next: usize,
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        for alignment in [1, 2, 4, 8, 16, 32, 64, 128, 256].iter() {
            for addr in 0..alignment * 10 {
                let aligned = align_up(addr, *alignment);

                assert_eq!(aligned % *alignment, 0);
                assert!(aligned >= addr);
            }
        }
    }
}
