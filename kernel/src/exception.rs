use x86_64::{
    registers::{control::Cr2, segmentation::GS},
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

use crate::{
    gsdata::KernelData,
    process::{Context, MANAGER},
    wrap,
};

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    unsafe { GS::swap() };

    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}

wrap!(irq(PageFaultErrorCode), page_fault_handler_inner => page_fault_handler);

extern "C" fn page_fault_handler_inner(context: &mut Context, error_code: PageFaultErrorCode) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        let address = Cr2::read();
        panic!(
            "\
Page Fault
Accessed Address: {address:?}
Error Code: {error_code:?}
{stack_frame:#?}",
        );
    } else {
        log::error!("userland page fault");
        let cpu_data = KernelData::load_gsbase().unwrap();
        MANAGER.get().unwrap().lock().kill_thread(cpu_data, context);
    }
}
