use super::{
    exception,
    hardware::{self, InterruptIndex},
    DOUBLE_FAULT_IST_INDEX,
};
use spin::Lazy;
use x86_64::{
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

#[cfg_attr(test, allow(unreachable_code))]
pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    #[allow(clippy::needless_pass_by_value)]
    fn general_handler(_stack_frame: InterruptStackFrame, index: u8, _error_code: Option<u64>) {
        todo!("handle irq {}", index)
    }

    #[cfg(test)]
    panic!("IDT can not be loaded in test mode");

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    idt.breakpoint.set_handler_fn(exception::breakpoint);
    idt.page_fault.set_handler_fn(exception::page_fault);

    let options = idt.double_fault.set_handler_fn(exception::double_fault);
    unsafe { options.set_stack_index(DOUBLE_FAULT_IST_INDEX) };

    idt[InterruptIndex::Timer as u8].set_handler_fn(hardware::timer_interrupt);
    idt[InterruptIndex::Keyboard as u8].set_handler_fn(hardware::keyboard_interrupt);
    idt[InterruptIndex::PrimaryAta as u8].set_handler_fn(hardware::primary_ata_interrupt);

    idt
});

pub fn load() {
    IDT.load();
}
