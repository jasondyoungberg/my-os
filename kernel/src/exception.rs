use x86_64::{
    instructions::hlt,
    registers::{control::Cr2, segmentation::GS},
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

use crate::{
    process::{Context, MANAGER},
    wrap,
};

wrap!(irq, division_error_handler_inner => division_error_handler);
extern "C" fn division_error_handler_inner(context: &mut Context) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel Division Error\n{stack_frame:#?}");
    } else {
        log::warn!("User Division Error\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(_stack_frame: InterruptStackFrame) {
    loop {
        hlt();
    }
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::info!("Breakpoint\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    log::info!("Overflow\n{:#?}", stack_frame);
}

wrap!(irq, bound_range_exceeded_handler_inner => bound_range_exceeded_handler);
extern "C" fn bound_range_exceeded_handler_inner(context: &mut Context) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel Bound Range Exceeded\n{stack_frame:#?}");
    } else {
        log::warn!("User Bound Range Exceeded\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

wrap!(irq, invalid_opcode_handler_inner => invalid_opcode_handler);
extern "C" fn invalid_opcode_handler_inner(context: &mut Context) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel Invalid Opcode\n{stack_frame:#?}");
    } else {
        log::warn!("User Invalid Opcode\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

wrap!(irq, device_not_available_handler_inner => device_not_available_handler);
extern "C" fn device_not_available_handler_inner(context: &mut Context) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel Device Not Available\n{stack_frame:#?}");
    } else {
        log::warn!("User Device Not Available\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    unsafe { GS::swap() };

    panic!("Double Fault\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("Invalid TSS ({error_code:#x})\n{stack_frame:#?}",);
}

pub extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("Segment Not Present ({error_code:#x})\n{stack_frame:#?}",);
}

wrap!(irq(u64), stack_segment_fault_handler_inner => stack_segment_fault_handler);
extern "C" fn stack_segment_fault_handler_inner(context: &mut Context, error_code: u64) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel Stack Segment Fault ({error_code:#x})\n{stack_frame:#?}");
    } else {
        log::warn!("User Stack Segment Fault ({error_code:#x})\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

wrap!(irq(u64), general_protection_fault_handler_inner => general_protection_fault_handler);
extern "C" fn general_protection_fault_handler_inner(context: &mut Context, error_code: u64) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel General Protection Fault ({error_code:#x})\n{stack_frame:#?}");
    } else {
        log::warn!("User General Protection Fault ({error_code:#x})\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

wrap!(irq(PageFaultErrorCode), page_fault_handler_inner => page_fault_handler);
extern "C" fn page_fault_handler_inner(context: &mut Context, error_code: PageFaultErrorCode) {
    let address = Cr2::read();
    let registers = &context.registers;
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!(
            "\
Kernel Page Fault
Accessed Address: {address:?}
Error Code: {error_code:?}
Registers: {registers:#?}
{stack_frame:#?}",
        );
    } else {
        log::warn!(
            "\
User Page Fault
Accessed Address: {address:?}
Error Code: {error_code:?}
Registers: {registers:#?}
{stack_frame:#?}",
        );
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

pub extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("x87 Floating Point\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("Alignment Check ({error_code:#x})\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("Machine Check\n{stack_frame:#?}");
}

wrap!(irq, simd_floating_point_handler_inner => simd_floating_point_handler);
extern "C" fn simd_floating_point_handler_inner(context: &mut Context) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel SIMD Floating Point\n{stack_frame:#?}");
    } else {
        log::warn!("User SIMD Floating Point\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}

pub extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    panic!("Virtualization\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn cp_protection_exception_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("CP Protection Exception ({error_code:#x})\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn hv_injection_exception_handler(stack_frame: InterruptStackFrame) {
    panic!("HV Injection Exception\n{stack_frame:#?}");
}

pub extern "x86-interrupt" fn vmm_communication_exception_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("VMM Communication Exception ({error_code:#x})\n{stack_frame:#?}");
}

wrap!(irq(u64), security_exception_handler_inner => security_exception_handler);
extern "C" fn security_exception_handler_inner(context: &mut Context, error_code: u64) {
    let stack_frame = &context.stack_frame;
    if stack_frame.code_segment.rpl() as u8 == 0 {
        panic!("Kernel Security Exception ({error_code:#x})\n{stack_frame:#?}");
    } else {
        log::warn!("User Security Exception ({error_code:#x})\n{stack_frame:#?}");
        MANAGER.get().unwrap().lock().kill_thread(context);
    }
}
