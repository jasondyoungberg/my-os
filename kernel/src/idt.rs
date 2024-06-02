use spin::Lazy;
use x86_64::{
    registers::{control::Cr2, segmentation::GS},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use crate::{gsdata::GsData, print, println};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    unsafe {
        idt.breakpoint.set_handler_fn(breakpoint);
        idt.page_fault.set_handler_fn(page_fault).set_stack_index(1);
        idt.double_fault
            .set_handler_fn(double_fault)
            .set_stack_index(2);
        idt[0x20].set_handler_fn(timer);
    }

    idt
});

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint(_stack_frame: InterruptStackFrame) {
    println!("Breakpoint");
}

extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!(
        "\
Page fault
Accessed Address: {:?}
Error code: {:?}
{:#?}",
        Cr2::read(),
        error_code,
        stack_frame
    );
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Double fault\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame) {
    unsafe { GS::swap() };
    let gsdata = GsData::load().unwrap();
    print!("{}.", gsdata.cpu_id);
    gsdata.lapic.signal_eoi();
    unsafe { GS::swap() };
}
