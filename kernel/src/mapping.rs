use crate::structures::paging::{Page, PageTable, PageTableFlags, PhysFrame};

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
