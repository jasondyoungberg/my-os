use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::{error, info};

pub extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    info!("Breakpoint");
}

pub extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("Double Fault");
}

pub extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let address = x86_64::registers::control::Cr2::read();

    panic!(
        "\
Page Fault
Accessed Address: {address:?}
Error Code: {error_code:?}"
    );
}
