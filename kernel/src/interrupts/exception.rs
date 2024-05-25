use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn double_fault(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("Double Fault");
}

pub extern "x86-interrupt" fn general_protection_fault(
    _stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("General Protection Fault ({error_code})");
}

pub extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let address = x86_64::registers::control::Cr2::read();

    error!(
        "\
Page Fault
Accessed Address: {address:?}
Error Code: {error_code:?}
{stack_frame:#?}"
    );

    // println!(
    //     "{}",
    //     Hexdump(&mut stack_frame.instruction_pointer.as_ptr::<u8>())
    // );

    panic!();
}
