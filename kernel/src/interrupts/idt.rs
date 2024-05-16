use crate::println;

use super::hardware::{self, InterruptIndex};

use super::DOUBLE_FAULT_IST_INDEX;
use spin::Lazy;
use x86_64::set_general_handler;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

#[cfg_attr(test, allow(unreachable_code))]
pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    use super::exception::*;

    fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
        todo!("handle irq {}", index)
    }

    #[cfg(test)]
    panic!("IDT can not be loaded in test mode");

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    idt.breakpoint.set_handler_fn(breakpoint_handler);
    let options = idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe { options.set_stack_index(DOUBLE_FAULT_IST_INDEX) };
    idt.page_fault.set_handler_fn(page_fault_handler);

    idt[InterruptIndex::Timer as u8].set_handler_fn(hardware::timer_interrupt);
    idt[InterruptIndex::Keyboard as u8].set_handler_fn(hardware::keyboard_interrupt);
    idt[InterruptIndex::PrimaryAta as u8].set_handler_fn(hardware::primary_ata_interrupt);

    idt
});

pub fn load() {
    IDT.load();
}
