use spin::Lazy;
use x86_64::{
    registers::segmentation::GS,
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    lapic,
    pics::{pics_handler, PICS_OFFSET},
};

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
        unsafe { GS::swap() };
        todo!("handle irq {index} ({error_code:?})\n{stack_frame:#?}");
    }

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    set_general_handler!(&mut idt, pics_handler, PICS_OFFSET..=PICS_OFFSET + 16);

    idt[lapic::TIMER_VECTOR].set_handler_fn(crate::lapic::handle_timer);

    idt
});
