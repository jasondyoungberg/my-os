use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

#[cfg_attr(test, allow(unreachable_code))]
static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    #[cfg(test)]
    panic!("IDT can not be loaded in test mode");

    let mut idt = InterruptDescriptorTable::new();

    macro_rules! set_handler {
        ($name:ident) => {
            idt.$name.set_handler_fn($name)
        };
        ($name:ident, $idx:expr) => {
            let opt = set_handler!($name);
            unsafe { opt.set_stack_index($idx) }
        };
    }

    set_handler!(divide_error);
    set_handler!(debug);
    set_handler!(non_maskable_interrupt);
    set_handler!(breakpoint);
    set_handler!(overflow);
    set_handler!(bound_range_exceeded);
    set_handler!(invalid_opcode);
    set_handler!(device_not_available);
    set_handler!(double_fault, DOUBLE_FAULT_IST_INDEX);
    set_handler!(invalid_tss);
    set_handler!(segment_not_present);
    set_handler!(stack_segment_fault);
    set_handler!(general_protection_fault);
    set_handler!(page_fault);
    set_handler!(x87_floating_point);
    set_handler!(alignment_check);
    set_handler!(machine_check);
    set_handler!(simd_floating_point);
    set_handler!(virtualization);
    set_handler!(cp_protection_exception);
    set_handler!(hv_injection_exception);
    set_handler!(vmm_communication_exception);
    set_handler!(security_exception);

    idt
});

pub fn load() {
    IDT.load();
}

macro_rules! handler {
    ($name:ident, $msg:literal) => {
        handler!($name, $msg, _, ());
    };
    ($name:ident, $msg:literal, !) => {
        handler!($name, $msg, _, !);
    };
    ($name:ident, $msg:literal, $err:ty) => {
        handler!($name, $msg, $err, ());
    };
    ($name:ident, $msg:literal, _, $ret:ty) => {
        extern "x86-interrupt" fn $name(stack_frame: InterruptStackFrame) -> $ret {
            panic!("EXCEPTION: {}\n{:#?}", $msg, stack_frame);
        }
    };
    ($name:ident, $msg:literal, $err:ty, $ret:ty) => {
        extern "x86-interrupt" fn $name(stack_frame: InterruptStackFrame, err_code: $err) -> $ret {
            panic!("EXCEPTION: {} ({:?})\n{:#?}", $msg, err_code, stack_frame);
        }
    };
}

handler!(divide_error, "DIVIDE BY ZERO");
handler!(debug, "DEBUG");
handler!(non_maskable_interrupt, "NON-MASKABLE INTERRUPT");
handler!(breakpoint, "BREAKPOINT");
handler!(overflow, "OVERFLOW");
handler!(bound_range_exceeded, "BOUND RANGE EXCEEDED");
handler!(invalid_opcode, "INVALID OPCODE");
handler!(device_not_available, "DEVICE NOT AVAILABLE");
handler!(double_fault, "DOUBLE FAULT", u64, !);
handler!(invalid_tss, "INVALID TSS", u64);
handler!(segment_not_present, "SEGMENT NOT PRESENT", u64);
handler!(stack_segment_fault, "STACK-SEGMENT FAULT", u64);
handler!(general_protection_fault, "GENERAL PROTECTION FAULT", u64);
handler!(page_fault, "PAGE FAULT", PageFaultErrorCode);
handler!(x87_floating_point, "x87 FLOATING POINT");
handler!(alignment_check, "ALIGNMENT CHECK", u64);
handler!(machine_check, "MACHINE CHECK", !);
handler!(simd_floating_point, "SIMD FLOATING POINT");
handler!(virtualization, "VIRTUALIZATION");
handler!(cp_protection_exception, "CP PROTECTION", u64);
handler!(hv_injection_exception, "HV INJECTION");
handler!(vmm_communication_exception, "VMM COMMUNICATION", u64);
handler!(security_exception, "SECURITY", u64);
