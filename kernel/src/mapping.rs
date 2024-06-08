use spin::{Lazy, RwLock};
use x86_64::{
    instructions::interrupts::without_interrupts,
    registers::control::Cr3,
    structures::paging::{
        page::PageRange, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
        PhysFrame,
    },
    PhysAddr, VirtAddr,
};

use crate::{allocation::frame::MyFrameAllocator, HHDM_RESPONSE};

pub static MEMORY_OFFSET: Lazy<u64> = Lazy::new(|| HHDM_RESPONSE.offset());

static KERNEL_MAPPER: Lazy<RwLock<OffsetPageTable>> = Lazy::new(|| {
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

    RwLock::new(unsafe { OffsetPageTable::new(l4_table, VirtAddr::new(*MEMORY_OFFSET)) })
});

pub unsafe fn new_page_table(frame: PhysFrame) -> &'static mut PageTable {
    let phys = frame.start_address();
    let virt = VirtAddr::new(phys.as_u64() + *MEMORY_OFFSET);
    let ptr = virt.as_mut_ptr::<PageTable>();
    let table = unsafe { &mut *ptr };

    let mapper = KERNEL_MAPPER.read();
    table.clone_from(mapper.level_4_table());

    table
}

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

pub unsafe fn map_pages(
    mapper: &mut OffsetPageTable,
    pages: PageRange,
    flags: PageTableFlags,
) -> PhysFrame {
    let frame = MyFrameAllocator.allocate_frame().unwrap();
    for page in pages {
        unsafe { map_page_to_frame(mapper, page, frame, flags) };
    }
    frame
}

pub unsafe fn map_kernel_page_to_frame(page: Page, frame: PhysFrame, flags: PageTableFlags) {
    without_interrupts(|| {
        let mut mapper = KERNEL_MAPPER.write();
        unsafe { map_page_to_frame(&mut mapper, page, frame, flags) }
    })
}

pub unsafe fn map_kernel_page(page: Page, flags: PageTableFlags) -> PhysFrame {
    without_interrupts(|| {
        let mut mapper = KERNEL_MAPPER.write();
        unsafe { map_page(&mut mapper, page, flags) }
    })
}

pub fn hhdm(phys: PhysAddr) -> VirtAddr {
    VirtAddr::new(phys.as_u64() + *MEMORY_OFFSET)
}

pub fn hhdm_reverse(virt: VirtAddr) -> PhysAddr {
    PhysAddr::new(virt.as_u64() - *MEMORY_OFFSET)
}
