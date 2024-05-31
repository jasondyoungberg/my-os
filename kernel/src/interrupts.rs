use crate::{println, structures::idt::InterruptStackFrame};

pub extern "x86-interrupt" fn breakpoint(_stack_frame: InterruptStackFrame) {
    println!("Breakpoint");
}

pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("Double fault\n{:#?}", stack_frame);
}
