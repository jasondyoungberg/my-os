use spin::Lazy;
use x86_64::{
    registers::segmentation::GS,
    set_general_handler,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    debug, exception,
    hardware::{
        lapic,
        pics::{pics_handler, PICS_OFFSET},
    },
};

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
        unsafe { GS::swap() };
        todo!("handle irq {index} ({error_code:?})\n{stack_frame:#?}");
    }

    let mut idt = InterruptDescriptorTable::new();

    set_general_handler!(&mut idt, general_handler);

    set_general_handler!(&mut idt, pics_handler, PICS_OFFSET..=PICS_OFFSET + 16);

    unsafe {
        idt.divide_error
            .set_handler_fn(exception::division_error_handler);
        idt.debug.set_handler_fn(debug::debug_handler);
        idt.non_maskable_interrupt
            .set_handler_fn(exception::non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(exception::breakpoint_handler);
        idt.overflow.set_handler_fn(exception::overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(exception::bound_range_exceeded_handler);
        idt.invalid_opcode
            .set_handler_fn(exception::invalid_opcode_handler);
        idt.device_not_available
            .set_handler_fn(exception::device_not_available_handler);
        idt.double_fault
            .set_handler_fn(exception::double_fault_handler)
            .set_stack_index(0);
        idt.invalid_tss
            .set_handler_fn(exception::invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(exception::segment_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(exception::stack_segment_fault_handler)
            .set_stack_index(1);
        idt.general_protection_fault
            .set_handler_fn(exception::general_protection_fault_handler)
            .set_stack_index(2);
        idt.page_fault
            .set_handler_fn(exception::page_fault_handler)
            .set_stack_index(3);
        idt.x87_floating_point
            .set_handler_fn(exception::x87_floating_point_handler);
        idt.alignment_check
            .set_handler_fn(exception::alignment_check_handler);
        idt.machine_check
            .set_handler_fn(exception::machine_check_handler);
        idt.simd_floating_point
            .set_handler_fn(exception::simd_floating_point_handler);
        idt.virtualization
            .set_handler_fn(exception::virtualization_handler);
        idt.cp_protection_exception
            .set_handler_fn(exception::cp_protection_exception_handler);
        idt.hv_injection_exception
            .set_handler_fn(exception::hv_injection_exception_handler);
        idt.vmm_communication_exception
            .set_handler_fn(exception::vmm_communication_exception_handler);
        idt.security_exception
            .set_handler_fn(exception::security_exception_handler);

        idt[lapic::TIMER_VECTOR].set_handler_fn(lapic::handle_timer);
    }
    idt
});

#[macro_export]
macro_rules! wrap {
    (push) => {"
        push r15; push r14; push r13; push r12;
        push r11; push r10; push r9;  push r8;
                  push rbp; push rdi; push rsi;
        push rdx; push rcx; push rbx; push rax;
    "};
    (pop) => {"
        pop rax; pop rbx; pop rcx; pop rdx;
        pop rsi; pop rdi; pop rbp;
        pop r8;  pop r9;  pop r10; pop r11;
        pop r12; pop r13; pop r14; pop r15;
    "};
    (irq, $i:ident => $w: ident) => {
        const _: unsafe extern "C" fn(&mut $crate::process::Context) = $i;

        #[naked]
        pub extern "x86-interrupt" fn $w(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame
        ) {
            unsafe {
                core::arch::asm!(
                    "swapgs",
                    $crate::wrap!(push),
                    "mov rdi, rsp",
                    "call {inner}",
                    $crate::wrap!(pop),
                    "swapgs",
                    "iretq",

                    inner = sym $i,
                    options(noreturn)
                )
            }
        }
    };
    (irq($c:ty), $i:ident => $w: ident) => {
        $crate::wrap!(irq($c)->(), $i => $w);
    };
    (irq($c:ty)->$r:ty, $i:ident => $w: ident) => {
        const _: unsafe extern "C" fn(&mut $crate::process::Context, $c) = $i;

        #[naked]
        pub extern "x86-interrupt" fn $w(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame,
            _error_code: $c
        ) {
            unsafe {
                core::arch::asm!(
                    "swapgs",
                    "push r14",
                    "mov r14, [rsp+8]",
                    "mov [rsp+8], r15",
                    "push r13",
                    "push r12",
                    "push r11",
                    "push r10",
                    "push r9",
                    "push r8",
                    "push rbp",
                    "push rdi",
                    "push rsi",
                    "push rdx",
                    "push rcx",
                    "push rbx",
                    "push rax",
                    "mov rdi, rsp",
                    "mov rsi, r14",
                    "call {inner}",
                    $crate::wrap!(pop),
                    "swapgs",
                    "iretq",

                    inner = sym $i,
                    options(noreturn)
                )
            }
        }
    };
    (syscall, $i:ident => $w: ident) => {
        const _: unsafe extern "C" fn(&mut $crate::process::Registers) = $i;

        #[naked]
        pub extern "x86-interrupt" fn $w(
            _stack_frame: x86_64::structures::idt::InterruptStackFrame
        ) {
            unsafe {
                core::arch::asm!(
                    "swapgs",
                    "mov gs:8, rsp",
                    "mov rsp, gs:16",
                    $crate::wrap!(push),
                    "mov rdi, rsp",
                    "call {inner}",
                    $crate::wrap!(pop),
                    "mov rsp, gs:8",
                    "swapgs",
                    "sysretq",

                    inner = sym $i,
                    options(noreturn)
                )
            }
        }
    };
}
