use crate::{
    allocation::frame::alloc_frame,
    instructions::flush_tlb_all,
    registers::Cr3,
    structures::paging::{Page, PageTable, PageTableFlags, PhysFrame},
};

pub fn init() {
    let (frame, _) = Cr3::read();
    let table = PageTable::from_frame(frame);

    table.iter_mut().skip(256).for_each(|entry| {
        if entry.is_unused() {
            entry.set_frame(alloc_frame());
        }
        entry.set_flags(PageTableFlags::WRITABLE | PageTableFlags::PRESENT);
    });
    flush_tlb_all();
}

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
}
