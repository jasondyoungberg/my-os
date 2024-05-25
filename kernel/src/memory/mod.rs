use bootloader_api::info::MemoryRegions;
use spin::{Mutex, Once};
use x86_64::{PhysAddr, VirtAddr};

mod frame_alloc;
mod heap_alloc;
mod paging;

pub use paging::active_level_4_table;

pub const KERNEL_DYNAMIC: u64 = 0xffff_8000_0000_0000;
pub const KERNEL_DYNAMIC_END: u64 = 0xffff_81ff_ffff_ffff;

pub const PHYSICAL_MEMORY_OFFSET: u64 = 0xffff_f800_0000_0000;

pub const MINI_STACK_SIZE: usize = 0x1_0000; // 64 KiB
pub const FULL_STACK_SIZE: usize = 0x10_0000; // 1 MiB

pub static FRAME_ALLOCATOR: Once<Mutex<frame_alloc::BootInfoFrameAllocator>> = Once::new();

pub fn init(regions: &'static MemoryRegions) {
    let l4_table = unsafe { active_level_4_table() };
    l4_table[0].set_unused(); // todo: make sure this is safe

    FRAME_ALLOCATOR.call_once(|| Mutex::new(frame_alloc::BootInfoFrameAllocator::new(regions)));
}

pub fn print() {
    let l4_table = unsafe { active_level_4_table() };
    crate::memory::paging::print_table(l4_table, 0);
}

pub unsafe fn phys_to_ptr<T>(ptr: PhysAddr) -> &'static mut T {
    let virt = VirtAddr::new(ptr.as_u64() + PHYSICAL_MEMORY_OFFSET);
    let ptr = virt.as_mut_ptr();
    unsafe { &mut *ptr }
}
