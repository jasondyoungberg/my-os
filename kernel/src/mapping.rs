use spin::{Lazy, Mutex};
use x86_64::{
    instructions::interrupts::without_interrupts,
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame,
    },
    VirtAddr,
};

use crate::{allocation::frame::MyFrameAllocator, HHDP_RESPONSE};

pub static MEMORY_OFFSET: Lazy<u64> = Lazy::new(|| HHDP_RESPONSE.offset());

static KERNEL_MAPPER: Lazy<Mutex<OffsetPageTable>> = Lazy::new(|| {
    let (l4_frame, _) = Cr3::read();
    let l4_phys = l4_frame.start_address();
    let l4_virt = VirtAddr::new(l4_phys.as_u64() + *MEMORY_OFFSET);
    let l4_ptr = l4_virt.as_mut_ptr::<PageTable>();
    let l4_table = unsafe { &mut *l4_ptr };

    l4_table.iter_mut().skip(256).for_each(|l4_entry| {
        if l4_entry.is_unused() {
            let l3_frame = MyFrameAllocator.allocate_frame().unwrap();
            let l3_phys = l3_frame.start_address();
            let l3_virt = VirtAddr::new(l3_phys.as_u64() + *MEMORY_OFFSET);
            let l3_ptr = l3_virt.as_mut_ptr::<PageTable>();
            let l3_table = unsafe { &mut *l3_ptr };
            l3_table.zero();
            l4_entry.set_frame(l3_frame, PageTableFlags::WRITABLE | PageTableFlags::PRESENT);
        }
    });

    Mutex::new(unsafe { OffsetPageTable::new(l4_table, VirtAddr::new(*MEMORY_OFFSET)) })
});

pub unsafe fn map_page_to_frame(
    mapper: &mut OffsetPageTable,
    page: Page,
    frame: PhysFrame,
    flags: PageTableFlags,
) {
    unsafe { mapper.map_to(page, frame, flags, &mut MyFrameAllocator) }
        .unwrap()
        .flush();
}

pub unsafe fn map_page(
    mapper: &mut OffsetPageTable,
    page: Page,
    flags: PageTableFlags,
) -> PhysFrame {
    let frame = MyFrameAllocator.allocate_frame().unwrap();
    unsafe { map_page_to_frame(mapper, page, frame, flags) };
    frame
}

pub unsafe fn map_kernel_page_to_frame(page: Page, frame: PhysFrame, flags: PageTableFlags) {
    without_interrupts(|| {
        let mut mapper = KERNEL_MAPPER.lock();
        unsafe { map_page_to_frame(&mut mapper, page, frame, flags) }
    })
}

pub unsafe fn map_kernel_page(page: Page, flags: PageTableFlags) -> PhysFrame {
    without_interrupts(|| {
        let mut mapper = KERNEL_MAPPER.lock();
        unsafe { map_page(&mut mapper, page, flags) }
    })
}
