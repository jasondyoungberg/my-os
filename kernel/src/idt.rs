use spin::Lazy;
use x86_64::{
    registers::segmentation::GS,
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{lapic, pics::PICS_OFFSET};

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
        unsafe { GS::swap() };
        todo!("handle irq {index} ({error_code:?})\n{stack_frame:#?}");
    }

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    idt[PICS_OFFSET].set_handler_fn(crate::pics::handle_irq0);
    idt[PICS_OFFSET + 1].set_handler_fn(crate::pics::handle_irq1);
    idt[PICS_OFFSET + 2].set_handler_fn(crate::pics::handle_irq2);
    idt[PICS_OFFSET + 3].set_handler_fn(crate::pics::handle_irq3);
    idt[PICS_OFFSET + 4].set_handler_fn(crate::pics::handle_irq4);
    idt[PICS_OFFSET + 5].set_handler_fn(crate::pics::handle_irq5);
    idt[PICS_OFFSET + 6].set_handler_fn(crate::pics::handle_irq6);
    idt[PICS_OFFSET + 7].set_handler_fn(crate::pics::handle_irq7);
    idt[PICS_OFFSET + 8].set_handler_fn(crate::pics::handle_irq8);
    idt[PICS_OFFSET + 9].set_handler_fn(crate::pics::handle_irq9);
    idt[PICS_OFFSET + 10].set_handler_fn(crate::pics::handle_irq10);
    idt[PICS_OFFSET + 11].set_handler_fn(crate::pics::handle_irq11);
    idt[PICS_OFFSET + 12].set_handler_fn(crate::pics::handle_irq12);
    idt[PICS_OFFSET + 13].set_handler_fn(crate::pics::handle_irq13);
    idt[PICS_OFFSET + 14].set_handler_fn(crate::pics::handle_irq14);
    idt[PICS_OFFSET + 15].set_handler_fn(crate::pics::handle_irq15);

    idt[lapic::TIMER_VECTOR].set_handler_fn(crate::lapic::handle_timer);

    idt
});
