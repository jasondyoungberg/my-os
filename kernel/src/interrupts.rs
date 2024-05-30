use crate::{idt::InterruptStackFrame, println, CONSOLE};

pub extern "x86-interrupt" fn breakpoint(_stack_frame: InterruptStackFrame) {
    println!("Breakpoint");
    if let Some(mut console) = CONSOLE.try_lock() {
        console.write_str("Breakpoint\n");
    };
}

pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("Double fault\n{:#?}", stack_frame);
}
