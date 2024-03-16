use core::ptr::addr_of;

use spin::Lazy;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

use crate::idt::DOUBLE_FAULT_IST_INDEX;

const STACK_SIZE: usize = 4096 * 5;

pub static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();

    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
        stack_start + STACK_SIZE as u64
    };

    tss
});
