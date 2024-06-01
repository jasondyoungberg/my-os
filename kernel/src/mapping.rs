use spin::{Lazy, Mutex};

use crate::{
    allocation::frame::{self, alloc_frame},
    dbg,
    instructions::{flush_tlb, flush_tlb_all, without_interrupts},
    registers::Cr3,
    structures::paging::{Page, PageTable, PageTableFlags, PhysFrame},
};

static KERNEL_L4_TABLE: Lazy<Mutex<&mut PageTable>> = Lazy::new(|| {
    let (frame, _) = Cr3::read();
    let table = PageTable::from_frame(frame);

    table.iter_mut().skip(256).for_each(|entry| {
        if entry.is_unused() {
            let frame = alloc_frame();
            let table = PageTable::from_frame(frame);
            table.set_empty();
            entry.set_frame(frame);
        }
        entry.set_flags(PageTableFlags::WRITABLE | PageTableFlags::PRESENT);
    });
    flush_tlb_all();
    Mutex::new(table)
});

pub fn map_page_to_frame(
    l4_table: &mut PageTable,
    page: Page,
    frame: PhysFrame,
    flags: PageTableFlags,
) {
    let l3_table = l4_table.next_table(page.p4_index(), flags);
    let l2_table = l3_table.next_table(page.p3_index(), flags);
    let l1_table = l2_table.next_table(page.p2_index(), flags);
    let entry = &mut l1_table[page.p1_index()];

    entry.set_frame(frame);
    entry.set_flags(flags);

    flush_tlb(page.start());
}

pub fn map_page(l4_table: &mut PageTable, page: Page, flags: PageTableFlags) -> PhysFrame {
    let frame = alloc_frame();
    map_page_to_frame(l4_table, page, frame, flags);
    frame
}

pub fn map_kernel_page_to_frame(page: Page, frame: PhysFrame, flags: PageTableFlags) {
    without_interrupts(|| {
        let mut l4_table = KERNEL_L4_TABLE.lock();
        map_page_to_frame(*l4_table, page, frame, flags)
    })
}

pub fn map_kernel_page(page: Page, flags: PageTableFlags) -> PhysFrame {
    without_interrupts(|| {
        let mut l4_table = KERNEL_L4_TABLE.lock();
        map_page(*l4_table, page, flags)
    })
}
