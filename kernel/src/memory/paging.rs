use spin::Mutex;
use x86_64::{structures::paging::PageTable, VirtAddr};

use crate::memory::PHYSICAL_MEMORY_OFFSET;

pub unsafe fn active_level_4_table() -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    unsafe { super::phys_to_ptr(level_4_table_frame.start_address()) }
}

pub fn print_table(table: &PageTable, level: usize) {
    let indent = "    ".repeat(level as usize);

    for (i, entry) in table.iter().enumerate() {
        if entry.is_unused() {
            continue;
        }

        println!("{indent} {i:3}: {entry:?}");

        if level == 3 {
            continue;
        }

        if entry
            .flags()
            .contains(x86_64::structures::paging::PageTableFlags::HUGE_PAGE)
        {
            return;
        }

        let phys = entry.frame().unwrap().start_address();

        let child_table = unsafe { super::phys_to_ptr(phys) };

        print_table(child_table, level + 1);
    }
}
