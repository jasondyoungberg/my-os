use x86_64::{
    registers::{control::Cr2, segmentation::GS},
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    unsafe { GS::swap() };

    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    unsafe { GS::swap() };

    let address = Cr2::read();

    panic!(
        "\
Page Fault
Accessed Address: {address:?}
Error Code: {error_code:?}
{stack_frame:#?}",
    );
}